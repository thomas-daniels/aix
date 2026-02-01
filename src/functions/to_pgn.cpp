#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void ToPgn(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, string_t>(args.data[0], result, args.size(), [&](string_t game) {
		diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		auto pgn_result = Game::to_pgn_string(data);
		auto pgn = UnwrapDecoded<std::string>(std::move(pgn_result), "to_pgn");
		return StringVector::AddString(result, pgn);
	});
}

} // namespace

void Register_ToPgn(ExtensionLoader &loader) {
	auto to_pgn_function = ScalarFunction("to_pgn", {LogicalType::BLOB}, LogicalType::VARCHAR, ToPgn);
	loader.RegisterFunction(to_pgn_function);
}

} // namespace duckdb