#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void IsValidMovedata(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, bool>(args.data[0], result, args.size(), [&](string_t game) {
		diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		return Game::is_valid_movedata(data);
	});
}

} // namespace

void Register_IsValidMovedata(ExtensionLoader &loader) {
	auto is_valid_movedata_function = ScalarFunction("is_valid_movedata", {}, LogicalType::BOOLEAN, IsValidMovedata);
	is_valid_movedata_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	loader.RegisterFunction(is_valid_movedata_function);
}

} // namespace duckdb