#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

template <bool CHECK_NULLS>
inline void ClocksToMoveTimes(DataChunk &args, ExpressionState &state, Vector &result) {
	const auto count = args.size();

	auto &clocks_vector = args.data[0];
	auto &clocks_child = ListVector::GetEntry(clocks_vector);
	UnifiedVectorFormat clocks_uvf;
	clocks_vector.ToUnifiedFormat(args.size(), clocks_uvf);
	auto clocks_entries = UnifiedVectorFormat::GetData<list_entry_t>(clocks_uvf);

	const auto clocks_size = ListVector::GetListSize(clocks_vector);
	clocks_child.Flatten(clocks_size);
	D_ASSERT(clocks_child.GetVectorType() == VectorType::FLAT_VECTOR);
	auto clocks_data = FlatVector::GetData<uint16_t>(clocks_child);

	auto &clocks_child_validity = FlatVector::Validity(clocks_child);

	UnifiedVectorFormat increments_uvf;
	args.data[1].ToUnifiedFormat(count, increments_uvf);
	auto increments_data = UnifiedVectorFormat::GetData<uint8_t>(increments_uvf);

	result.SetVectorType(VectorType::FLAT_VECTOR);
	auto &results_child = ListVector::GetEntry(result);
	auto results_entries = FlatVector::GetData<list_entry_t>(result);
	auto &result_validity = FlatVector::Validity(result);

	for (idx_t i = 0; i < count; i++) {
		auto inc_idx = increments_uvf.sel->get_index(i);
		auto clocks_idx = clocks_uvf.sel->get_index(i);

		auto result_current_size = ListVector::GetListSize(result);

		if (!increments_uvf.validity.RowIsValid(inc_idx) || !clocks_uvf.validity.RowIsValid(clocks_idx)) {
			results_entries[i].offset = result_current_size;
			results_entries[i].length = 0;
			result_validity.SetInvalid(i);
			continue;
		}

		const auto clocks_length = clocks_entries[clocks_idx].length;
		if (clocks_length < 2) {
			results_entries[i].offset = result_current_size;
			results_entries[i].length = 0;
		} else {
			const auto clocks_offset = clocks_entries[clocks_idx].offset;

			if constexpr (CHECK_NULLS) {
				ValidityMask range_mask(clocks_length);
				range_mask.Slice(clocks_child_validity, clocks_offset, clocks_length);
				if (!range_mask.CheckAllValid(clocks_length)) {
					throw InvalidInputException("clocks_to_move_times: clock list cannot contain NULL values");
				}
			}

			ListVector::Reserve(result, result_current_size + clocks_length - 1);
			auto results_data = FlatVector::GetData<uint16_t>(results_child);
			results_entries[i].offset = result_current_size;
			results_entries[i].length = clocks_length - 1;

			auto prev = static_cast<int32_t>(clocks_data[clocks_offset]);
			for (idx_t j = 1; j < clocks_length; j++) {
				const auto curr = static_cast<int32_t>(clocks_data[clocks_offset + j]);

				const auto move_time = static_cast<uint16_t>(std::max(0, prev - curr + increments_data[inc_idx]));
				// Because the difference could be negative because of +15s on Lichess, this is clamped to 0.
				// A negative move time does not make sense and we cannot reconstruct the actual move time.

				results_data[result_current_size + j - 1] = move_time;

				prev = curr;
			}
			ListVector::SetListSize(result, result_current_size + clocks_length - 1);
		}
	}
}

} // namespace

void Register_ClocksToMoveTimes(ExtensionLoader &loader) {
	auto clocks_to_move_times_function =
	    ScalarFunction("clocks_to_move_times", {LogicalType::LIST(LogicalType::USMALLINT), LogicalType::UTINYINT},
	                   LogicalType::LIST(LogicalType::USMALLINT), ClocksToMoveTimes<false>);
	loader.RegisterFunction(clocks_to_move_times_function);

	auto clocks_to_move_times_check_nulls_function = ScalarFunction(
	    "clocks_to_move_times__check_nulls", {LogicalType::LIST(LogicalType::USMALLINT), LogicalType::UTINYINT},
	    LogicalType::LIST(LogicalType::USMALLINT), ClocksToMoveTimes<true>);
	loader.RegisterFunction(clocks_to_move_times_check_nulls_function);
}

} // namespace duckdb