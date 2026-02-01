#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void ToUci(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, string_t>(args.data[0], result, args.size(), [&](string_t game) {
		diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		auto uci_result = Game::to_uci_string(data);
		auto uci = UnwrapDecoded<std::string>(std::move(uci_result), "to_uci");
		return StringVector::AddString(result, uci);
	});
}

} // namespace

void Register_ToUci(ExtensionLoader &loader) {
	auto to_uci_function = ScalarFunction("to_uci", {LogicalType::BLOB}, LogicalType::VARCHAR, ToUci);
	loader.RegisterFunction(to_uci_function);
}

} // namespace duckdb