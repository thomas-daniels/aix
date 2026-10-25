#include "aixchess_functions.hpp"

#include <cstdint>
#include <cstddef>

namespace duckdb {

struct PieceCountSquares {
	bool valid;

	Bitboards bb;

	static void AssignResult(Vector &result, idx_t i, PieceCountSquares value) {
		if (!value.valid) {
			FlatVector::SetNull(result, i, true);
			return;
		}

		auto &entries = StructVector::GetEntries(result);
		FlatVector::GetDataMutable<uint8_t>(entries[0])[i] = static_cast<uint8_t>(bits::popcount(value.bb.w_q));
		FlatVector::GetDataMutable<uint8_t>(entries[1])[i] = static_cast<uint8_t>(bits::popcount(value.bb.w_r));
		FlatVector::GetDataMutable<uint8_t>(entries[2])[i] = static_cast<uint8_t>(bits::popcount(value.bb.w_b));
		FlatVector::GetDataMutable<uint8_t>(entries[3])[i] = static_cast<uint8_t>(bits::popcount(value.bb.w_n));
		FlatVector::GetDataMutable<uint8_t>(entries[4])[i] = static_cast<uint8_t>(bits::popcount(value.bb.w_p));
		FlatVector::GetDataMutable<uint8_t>(entries[5])[i] = static_cast<uint8_t>(bits::popcount(value.bb.b_q));
		FlatVector::GetDataMutable<uint8_t>(entries[6])[i] = static_cast<uint8_t>(bits::popcount(value.bb.b_r));
		FlatVector::GetDataMutable<uint8_t>(entries[7])[i] = static_cast<uint8_t>(bits::popcount(value.bb.b_b));
		FlatVector::GetDataMutable<uint8_t>(entries[8])[i] = static_cast<uint8_t>(bits::popcount(value.bb.b_n));
		FlatVector::GetDataMutable<uint8_t>(entries[9])[i] = static_cast<uint8_t>(bits::popcount(value.bb.b_p));
	}
};

inline void PieceCountsAtPosition(DataChunk &args, ExpressionState &state, Vector &result) {
	GenericExecutor::ExecuteBinary<PrimitiveType<string_t>, PrimitiveType<int32_t>, PieceCountSquares>(
	    args.data[0], args.data[1], result, args.size(), [&](PrimitiveType<string_t> game, PrimitiveType<int32_t> pos) {
		    PieceCountSquares pcs;
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto bitboards_result = Game::pieces_at_position(data, pos.val);
		    const auto bitboards_opt =
		        UnwrapOptionalDecoded<Bitboards>(std::move(bitboards_result), "piece_counts_at_position");

		    if (!bitboards_opt.has_value()) {
			    pcs.valid = false;
			    return pcs;
		    }

		    pcs.valid = true;
		    pcs.bb = *bitboards_opt;

		    return pcs;
	    });
}

void Register_PieceCountsAtPosition(ExtensionLoader &loader) {
	child_list_t<LogicalType> piece_counts_children;
	piece_counts_children.push_back(std::make_pair("wQ", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("wR", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("wB", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("wN", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("wP", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("bQ", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("bR", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("bB", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("bN", LogicalType::UTINYINT));
	piece_counts_children.push_back(std::make_pair("bP", LogicalType::UTINYINT));

	auto piece_counts_pos_function = ScalarFunction("piece_counts_at_position", {},
	                                                LogicalType::STRUCT(piece_counts_children), PieceCountsAtPosition);
	piece_counts_pos_function.GetSignature()
	    .AddParameter("movedata", LogicalType::BLOB)
	    .AddParameter("position", LogicalType::INTEGER);
	piece_counts_pos_function.SetFallible();
	RegisterFunc(loader, piece_counts_pos_function,
	             "Returns the piece counts at a given position. The starting position is 0, so the final position "
	             "is ply_count. Negative integers are accepted, but positive integers is typically faster.",
	             "piece_counts_at_position('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB, 6)");
}

} // namespace duckdb