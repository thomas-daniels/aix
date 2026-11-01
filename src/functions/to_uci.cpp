#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void ToUci(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, string_t>(args.data[0], result, args.size(), [&](string_t game) {
		auto data = game_to_span(game);
		auto uci_result = Game::to_uci_string(data);
		auto uci = UnwrapDecoded<std::string>(std::move(uci_result), "to_uci");
		return StringVector::AddString(result, uci);
	});
}

} // namespace

void Register_ToUci(ExtensionLoader &loader) {
	auto to_uci_function = ScalarFunction("to_uci", {}, LogicalType::VARCHAR, ToUci);
	to_uci_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	to_uci_function.SetFallible();
	RegisterFunc(loader, to_uci_function, "Represents the game as a UCI string, e.g. e2e4 e7e5 f1c4 b8c6 g1f3 ...",
	             "to_uci('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");
}

} // namespace duckdb