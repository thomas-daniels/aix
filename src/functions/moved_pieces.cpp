#include "aixchess_functions.hpp"

#include <algorithm>

namespace duckdb {

namespace {

inline void MovedPiecesList(DataChunk &args, ExpressionState &state, Vector &result) {
	GenericExecutor::ExecuteUnary<PrimitiveType<string_t>, GenericListType<PrimitiveType<string_t>>>(
	    args.data[0], result, args.size(), [&](PrimitiveType<string_t> game) {
		    auto data = game_to_span(game.val);
		    auto pieces_result = Game::moved_pieces(data);
		    const auto pieces = UnwrapDecoded<std::string>(std::move(pieces_result), "moved_pieces_list");
		    const std::vector<char> chars(pieces.begin(), pieces.end());

		    GenericListType<PrimitiveType<string_t>> result;
		    for (auto &c : pieces) {
			    result.values.push_back(PrimitiveType<string_t>(std::string(1, c)));
		    }
		    return result;
	    });
}

inline void MovedPieces(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<string_t, string_t>(args.data[0], result, args.size(), [&](string_t game) {
		auto data = game_to_span(game);
		auto pieces_result = Game::moved_pieces(data);
		const auto pieces = UnwrapDecoded<std::string>(std::move(pieces_result), "moved_pieces");
		return StringVector::AddString(result, pieces);
	});
}

} // namespace

void Register_MovedPieces(ExtensionLoader &loader) {
	auto moved_pieces_list_function =
	    ScalarFunction("moved_pieces_list", {}, LogicalType::LIST(LogicalType::VARCHAR), MovedPiecesList);
	moved_pieces_list_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	moved_pieces_list_function.SetFallible();
	RegisterFunc(loader, moved_pieces_list_function,
	             "Returns the moved pieces in order as a list, e.g. [P, p, B, n, N, ...]",
	             "moved_pieces_list('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");

	auto moved_pieces_function = ScalarFunction("moved_pieces", {}, LogicalType::VARCHAR, MovedPieces);
	moved_pieces_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	moved_pieces_function.SetFallible();
	RegisterFunc(loader, moved_pieces_function, "Returns the moved pieces in order as a string, e.g. PpBnN...",
	             "moved_pieces('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");
}

} // namespace duckdb