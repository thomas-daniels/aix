#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void Recompress(DataChunk &args, ExpressionState &state, Vector &result) {
	BinaryExecutor::Execute<string_t, uint8_t, string_t>(
	    args.data[0], args.data[1], result, args.size(), [&](string_t game, uint8_t level) {
		    if (level > 2) {
			    throw InvalidInputException("Invalid compression level %d: must be 0, 1, or 2", level);
		    }

		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		    std::vector<uint8_t> buffer(game.GetSize() * 16); // allocate more than enough space
		    diplomat::span<uint8_t> buffer_span = {buffer.data(), buffer.size()};

		    auto written = UnwrapDecoded<size_t>(Game::recompress(data, level, buffer_span), "recompress");

		    return StringVector::AddStringOrBlob(result, reinterpret_cast<const char *>(buffer.data()), written);
	    });
}

} // namespace

void Register_Recompress(ExtensionLoader &loader) {
	auto recompress_function =
	    ScalarFunction("recompress", {LogicalType::BLOB, LogicalType::UTINYINT}, LogicalType::BLOB, Recompress);
	loader.RegisterFunction(recompress_function);
}

} // namespace duckdb