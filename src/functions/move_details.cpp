#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

const char *SQUARES[] = {"a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2",
                         "f2", "g2", "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4",
                         "c4", "d4", "e4", "f4", "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5",
                         "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a7", "b7", "c7", "d7",
                         "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"};

const int PLY_I = 0;
const int ROLE_I = 1;
const int FROM_I = 2;
const int TO_I = 3;
const int PROMOTION_I = 4;
const int CAPTURE_I = 5;
const int IS_CASTLE_I = 6;
const int CHECK_I = 7;
const int CHECKMATE_I = 8;
const int IS_EN_PASSANT_I = 9;

template <bool NULLABLE>
struct MoveDetailsStruct {
	MoveDetails inner;
	bool valid = true;

	static void AssignResult(Vector &result, idx_t i, MoveDetailsStruct value) {
		if constexpr (NULLABLE) {
			if (!value.valid) {
				FlatVector::SetNull(result, i, true);
				return;
			}
		}

		auto &entries = StructVector::GetEntries(result);

		FlatVector::GetData<uint16_t>(*entries[PLY_I])[i] = value.inner.ply;
		FlatVector::GetData<string_t>(*entries[ROLE_I])[i] = std::string(1, value.inner.role);
		FlatVector::GetData<string_t>(*entries[FROM_I])[i] = SQUARES[value.inner.from];
		FlatVector::GetData<string_t>(*entries[TO_I])[i] = SQUARES[value.inner.to];
		FlatVector::GetData<string_t>(*entries[PROMOTION_I])[i] =
		    value.inner.promotion == 0 ? "" : std::string(1, value.inner.promotion);
		FlatVector::GetData<string_t>(*entries[CAPTURE_I])[i] =
		    value.inner.capture == 0 ? "" : std::string(1, value.inner.capture);
		FlatVector::GetData<bool>(*entries[IS_CASTLE_I])[i] = value.inner.is_castle;
		FlatVector::GetData<bool>(*entries[CHECK_I])[i] = value.inner.is_check;
		FlatVector::GetData<bool>(*entries[CHECKMATE_I])[i] = value.inner.is_checkmate;
		FlatVector::GetData<bool>(*entries[IS_EN_PASSANT_I])[i] = value.inner.is_en_passant;
	}
}; // namespace

inline void MoveDetailsFn(DataChunk &args, ExpressionState &state, Vector &result) {
	GenericExecutor::ExecuteUnary<PrimitiveType<string_t>, GenericListType<MoveDetailsStruct<false>>>(
	    args.data[0], result, args.size(), [&](PrimitiveType<string_t> game) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto game_obj_result = Game::from_bytes(data);
		    auto game_obj = UnwrapDecoded(std::move(game_obj_result), "move_details");
		    auto iter = game_obj->move_details_iterator();
		    GenericListType<MoveDetailsStruct<false>> moves;
		    while (auto opt = UnwrapOptionalDecoded(iter->next(), "move_details")) {
			    MoveDetailsStruct<false> move;
			    move.inner = *opt;
			    moves.values.push_back(move);
		    }

		    return moves;
	    });
}

inline void MoveDetailsAtFn(DataChunk &args, ExpressionState &state, Vector &result) {
	GenericExecutor::ExecuteBinary<PrimitiveType<string_t>, PrimitiveType<int16_t>, MoveDetailsStruct<true>>(
	    args.data[0], args.data[1], result, args.size(), [&](PrimitiveType<string_t> game, PrimitiveType<int16_t> ply) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto game_obj_result = Game::from_bytes(data);
		    auto game_obj = UnwrapDecoded(std::move(game_obj_result), "move_details_at");
		    auto iter = game_obj->move_details_iterator();
		    auto maybe_move_result = iter->nth(ply.val);
		    auto maybe_move = UnwrapOptionalDecoded(std::move(maybe_move_result), "move_details_at");

		    MoveDetailsStruct<true> move;
		    if (!maybe_move.has_value()) {
			    move.valid = false;
		    } else {
			    move.inner = *maybe_move;
		    }
		    return move;
	    });
}

} // namespace

void Register_MoveDetails(ExtensionLoader &loader) {
	child_list_t<LogicalType> move_children;
	move_children.push_back(std::make_pair("ply", LogicalType::USMALLINT));
	move_children.push_back(std::make_pair("role", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("from", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("to", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("promotion", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("capture", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("is_castle", LogicalType::BOOLEAN));
	move_children.push_back(std::make_pair("is_check", LogicalType::BOOLEAN));
	move_children.push_back(std::make_pair("is_checkmate", LogicalType::BOOLEAN));
	move_children.push_back(std::make_pair("is_en_passant", LogicalType::BOOLEAN));

	auto move_details_function = ScalarFunction("move_details", {LogicalType::BLOB},
	                                            LogicalType::LIST(LogicalType::STRUCT(move_children)), MoveDetailsFn);
	loader.RegisterFunction(move_details_function);

	auto move_details_at_function = ScalarFunction("move_details_at", {LogicalType::BLOB, LogicalType::SMALLINT},
	                                               LogicalType::STRUCT(move_children), MoveDetailsAtFn);
	loader.RegisterFunction(move_details_at_function);
}

} // namespace duckdb