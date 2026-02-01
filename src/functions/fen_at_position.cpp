#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void FenAtPosition(DataChunk &args, ExpressionState &state, Vector &result) {
	BinaryExecutor::ExecuteWithNulls<string_t, int32_t, string_t>(
	    args.data[0], args.data[1], result, args.size(),
	    [&](string_t game, int32_t pos, ValidityMask &mask, idx_t idx) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		    auto fen_result = Game::fen_at_position(data, pos);
		    auto fen_opt = UnwrapOptionalDecoded<std::string>(std::move(fen_result), "fen_at_position");

		    if (!fen_opt.has_value()) {
			    mask.SetInvalid(idx);
			    return string_t();
		    }

		    return StringVector::AddString(result, *fen_opt);
	    });
}

} // namespace

void Register_FenAtPosition(ExtensionLoader &loader) {
	auto fen_pos_function = ScalarFunction("fen_at_position", {LogicalType::BLOB, LogicalType::INTEGER},
	                                       LogicalType::VARCHAR, FenAtPosition);
	loader.RegisterFunction(fen_pos_function);
}

} // namespace duckdb