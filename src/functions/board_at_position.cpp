#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

struct BoardStruct {
	int8_t board[64] = {0};
	bool valid;

	static void AssignResult(Vector &result, idx_t i, BoardStruct value) {
		if (!value.valid) {
			FlatVector::SetNull(result, i, true);
			return;
		}

		auto &entries = StructVector::GetEntries(result);
		for (auto j = 0; j < 64; j++) {
			auto data = FlatVector::GetData<string_t>(*entries[j]);

			auto c = value.board[j];
			std::string p;
			if (c == 0) {
				p = "";
			} else {
				p = std::string(1, c);
			}

			data[i] = p;
			// no need for a StringVector::AddString call here, because the strings are so short that they will always
			// be inlined
		}
	}
};

inline void BoardAtPosition(DataChunk &args, ExpressionState &state, Vector &result) {
	GenericExecutor::ExecuteBinary<PrimitiveType<string_t>, PrimitiveType<int32_t>, BoardStruct>(
	    args.data[0], args.data[1], result, args.size(), [&](PrimitiveType<string_t> game, PrimitiveType<int32_t> pos) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};
		    BoardStruct b;
		    auto board_span = diplomat::span<int8_t>(b.board, 64);
		    auto result = Game::board_at_position(data, pos.val, board_span);
		    auto opt = UnwrapOptionalDecoded<std::monostate>(std::move(result), "board_at_position");
		    if (opt.has_value()) {
			    b.valid = true;
		    } else {
			    b.valid = false;
		    }
		    return b;
	    });
}

} // namespace

void Register_BoardAtPosition(ExtensionLoader &loader) {
	child_list_t<LogicalType> board_children;
	const char *squares[] = {"a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2",
	                         "f2", "g2", "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4",
	                         "c4", "d4", "e4", "f4", "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5",
	                         "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a7", "b7", "c7", "d7",
	                         "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"};
	for (auto sq : squares) {
		board_children.push_back(std::make_pair(sq, LogicalType::VARCHAR));
	}
	auto board_pos_function = ScalarFunction("board_at_position", {LogicalType::BLOB, LogicalType::INTEGER},
	                                         LogicalType::STRUCT(board_children), BoardAtPosition);
	loader.RegisterFunction(board_pos_function);
}

} // namespace duckdb