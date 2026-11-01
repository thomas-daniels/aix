#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

inline void FenAtPosition(DataChunk &args, ExpressionState &state, Vector &result) {
	BinaryExecutor::Execute<string_t, int32_t, string_t>(
	    args.data[0], args.data[1], result, args.size(), [&](string_t game, int32_t pos) -> std::optional<string_t> {
		    auto data = game_to_span(game);
		    auto fen_result = Game::fen_at_position(data, pos);
		    auto fen_opt = UnwrapOptionalDecoded<std::string>(std::move(fen_result), "fen_at_position");

		    if (!fen_opt.has_value()) {
			    return std::nullopt;
		    }

		    return StringVector::AddString(result, *fen_opt);
	    });
}

} // namespace

void Register_FenAtPosition(ExtensionLoader &loader) {
	auto fen_pos_function = ScalarFunction("fen_at_position", {}, LogicalType::VARCHAR, FenAtPosition);
	fen_pos_function.GetSignature()
	    .AddParameter("movedata", LogicalType::BLOB)
	    .AddParameter("position", LogicalType::INTEGER);
	fen_pos_function.SetFallible();
	RegisterFunc(loader, fen_pos_function,
	             "Returns the FEN at a given position. The starting position is 0, so the final position is "
	             "ply_count. Negative integers are accepted, but positive integers is typically faster.",
	             "fen_at_position('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB, 6)");
}

} // namespace duckdb