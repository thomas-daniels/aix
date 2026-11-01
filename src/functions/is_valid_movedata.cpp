#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void IsValidMovedata(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, bool>(args.data[0], result, args.size(), [&](string_t game) {
		auto data = game_to_span(game);
		return Game::is_valid_movedata(data);
	});
}

} // namespace

void Register_IsValidMovedata(ExtensionLoader &loader) {
	auto is_valid_movedata_function = ScalarFunction("is_valid_movedata", {}, LogicalType::BOOLEAN, IsValidMovedata);
	is_valid_movedata_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	RegisterFunc(loader, is_valid_movedata_function,
	             "Returns true if movedata represents a valid chess game, false otherwise.",
	             "is_valid_movedata('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");
}

} // namespace duckdb