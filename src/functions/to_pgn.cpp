#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void ToPgn(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, string_t>(args.data[0], result, args.size(), [&](string_t game) {
		auto data = game_to_span(game);
		auto pgn_result = Game::to_pgn_string(data);
		auto pgn = UnwrapDecoded<std::string>(std::move(pgn_result), "to_pgn");
		return StringVector::AddString(result, pgn);
	});
}

} // namespace

void Register_ToPgn(ExtensionLoader &loader) {
	auto to_pgn_function = ScalarFunction("to_pgn", {}, LogicalType::VARCHAR, ToPgn);
	to_pgn_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	to_pgn_function.SetFallible();
	RegisterFunc(loader, to_pgn_function, "Represents the game as a PGN string, e.g. 1. e4 e5 2. Bc4 Nc6 3. Nf3 ...",
	             "to_pgn('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");
}

} // namespace duckdb