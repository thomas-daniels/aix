#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void LichessTimeControl(DataChunk &args, ExpressionState &state, Vector &result) {
	BinaryExecutor::Execute<uint16_t, uint8_t, string_t>(args.data[0], args.data[1], result, args.size(),
	                                                     [&](uint16_t initial, uint8_t increment) {
		                                                     const auto total = initial + increment * 40;
		                                                     auto tc = total < 30     ? "Ultrabullet"
		                                                               : total < 180  ? "Bullet"
		                                                               : total < 480  ? "Blitz"
		                                                               : total < 1500 ? "Rapid"
		                                                                              : "Classical";
		                                                     return StringVector::AddString(result, tc);
	                                                     });
}

} // namespace

void Register_LichessTimeControl(ExtensionLoader &loader) {
	auto lichess_time_control_function =
	    ScalarFunction("time_control_lichess", {LogicalType::USMALLINT, LogicalType::UTINYINT}, LogicalType::VARCHAR,
	                   LichessTimeControl);
	loader.RegisterFunction(lichess_time_control_function);
}

} // namespace duckdb