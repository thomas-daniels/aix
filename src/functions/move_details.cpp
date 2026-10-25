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
const int IS_EN_PASSANT_I = 7;
const int CHECK_I = 8;
const int CHECKMATE_I = 9;
const int STALEMATE_I = 10;
const int LEGAL_RESPONSE_MOVE_COUNT_I = 11;

template <bool NULLABLE, typename INNER_TYPE>
struct MoveDetailsStruct {
	INNER_TYPE inner;
	bool valid = true;

	static void AssignResult(Vector &result, idx_t i, MoveDetailsStruct value) {
		if constexpr (NULLABLE) {
			if (!value.valid) {
				FlatVector::SetNull(result, i, true);
				return;
			}
		}

		auto &entries = StructVector::GetEntries(result);

		FlatVector::GetDataMutable<uint16_t>(entries[PLY_I])[i] = value.inner.ply;
		FlatVector::GetDataMutable<string_t>(entries[ROLE_I])[i] = std::string(1, value.inner.role);
		FlatVector::GetDataMutable<string_t>(entries[FROM_I])[i] = SQUARES[value.inner.from];
		FlatVector::GetDataMutable<string_t>(entries[TO_I])[i] = SQUARES[value.inner.to];
		FlatVector::GetDataMutable<string_t>(entries[PROMOTION_I])[i] =
		    value.inner.promotion == 0 ? "" : std::string(1, value.inner.promotion);
		FlatVector::GetDataMutable<string_t>(entries[CAPTURE_I])[i] =
		    value.inner.capture == 0 ? "" : std::string(1, value.inner.capture);
		FlatVector::GetDataMutable<bool>(entries[IS_CASTLE_I])[i] = value.inner.is_castle;
		FlatVector::GetDataMutable<bool>(entries[IS_EN_PASSANT_I])[i] = value.inner.is_en_passant;
		FlatVector::GetDataMutable<bool>(entries[CHECK_I])[i] = value.inner.is_check;
		FlatVector::GetDataMutable<bool>(entries[CHECKMATE_I])[i] = value.inner.is_checkmate;
		if constexpr (std::is_same_v<INNER_TYPE, MoveDetailsExtended>) {
			FlatVector::GetDataMutable<bool>(entries[STALEMATE_I])[i] = value.inner.is_stalemate;
			FlatVector::GetDataMutable<uint8_t>(entries[LEGAL_RESPONSE_MOVE_COUNT_I])[i] =
			    value.inner.legal_response_move_count;
		}
	}
}; // namespace

inline void MoveDetailsExtFn(DataChunk &args, ExpressionState &state, Vector &result) {
	const char *func_name = "move_details_ext";

	GenericExecutor::ExecuteUnary<PrimitiveType<string_t>,
	                              GenericListType<MoveDetailsStruct<false, MoveDetailsExtended>>>(
	    args.data[0], result, args.size(), [&](PrimitiveType<string_t> game) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto game_obj_result = Game::from_bytes(data);
		    auto game_obj = UnwrapDecoded(std::move(game_obj_result), func_name);
		    auto iter = game_obj->move_details_ext_iterator();
		    GenericListType<MoveDetailsStruct<false, MoveDetailsExtended>> moves;
		    while (auto opt = UnwrapOptionalDecoded(iter->next(), func_name)) {
			    MoveDetailsStruct<false, MoveDetailsExtended> move;
			    move.inner = *opt;
			    moves.values.push_back(move);
		    }

		    return moves;
	    });
}

inline void MoveDetailsFn(DataChunk &args, ExpressionState &state, Vector &result) {
	const char *func_name = "move_details";

	GenericExecutor::ExecuteUnary<PrimitiveType<string_t>, GenericListType<MoveDetailsStruct<false, MoveDetails>>>(
	    args.data[0], result, args.size(), [&](PrimitiveType<string_t> game) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto game_obj_result = Game::from_bytes(data);
		    auto game_obj = UnwrapDecoded(std::move(game_obj_result), func_name);
		    auto iter = game_obj->move_details_iterator();
		    GenericListType<MoveDetailsStruct<false, MoveDetails>> moves;
		    while (auto opt = UnwrapOptionalDecoded(iter->next(), func_name)) {
			    MoveDetailsStruct<false, MoveDetails> move;
			    move.inner = *opt;
			    moves.values.push_back(move);
		    }

		    return moves;
	    });
}

inline void MoveDetailsExtAtFn(DataChunk &args, ExpressionState &state, Vector &result) {
	const char *func_name = "move_details_ext_at";

	GenericExecutor::ExecuteBinary<PrimitiveType<string_t>, PrimitiveType<int16_t>,
	                               MoveDetailsStruct<true, MoveDetailsExtended>>(
	    args.data[0], args.data[1], result, args.size(), [&](PrimitiveType<string_t> game, PrimitiveType<int16_t> ply) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto game_obj_result = Game::from_bytes(data);
		    auto game_obj = UnwrapDecoded(std::move(game_obj_result), func_name);
		    auto iter = game_obj->move_details_ext_iterator();
		    auto maybe_move_result = iter->nth(ply.val);
		    auto maybe_move = UnwrapOptionalDecoded(std::move(maybe_move_result), func_name);

		    MoveDetailsStruct<true, MoveDetailsExtended> move;
		    if (!maybe_move.has_value()) {
			    move.valid = false;
		    } else {
			    move.inner = *maybe_move;
		    }
		    return move;
	    });
}

inline void MoveDetailsAtFn(DataChunk &args, ExpressionState &state, Vector &result) {
	const char *func_name = "move_details_at";

	GenericExecutor::ExecuteBinary<PrimitiveType<string_t>, PrimitiveType<int16_t>,
	                               MoveDetailsStruct<true, MoveDetails>>(
	    args.data[0], args.data[1], result, args.size(), [&](PrimitiveType<string_t> game, PrimitiveType<int16_t> ply) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};

		    auto game_obj_result = Game::from_bytes(data);
		    auto game_obj = UnwrapDecoded(std::move(game_obj_result), func_name);
		    auto iter = game_obj->move_details_iterator();
		    auto maybe_move_result = iter->nth(ply.val);
		    auto maybe_move = UnwrapOptionalDecoded(std::move(maybe_move_result), func_name);

		    MoveDetailsStruct<true, MoveDetails> move;
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
	child_list_t<LogicalType> move_children_ext;
	move_children_ext.push_back(std::make_pair("ply", LogicalType::USMALLINT));
	move_children_ext.push_back(std::make_pair("role", LogicalType::VARCHAR));
	move_children_ext.push_back(std::make_pair("from", LogicalType::VARCHAR));
	move_children_ext.push_back(std::make_pair("to", LogicalType::VARCHAR));
	move_children_ext.push_back(std::make_pair("promotion", LogicalType::VARCHAR));
	move_children_ext.push_back(std::make_pair("capture", LogicalType::VARCHAR));
	move_children_ext.push_back(std::make_pair("is_castle", LogicalType::BOOLEAN));
	move_children_ext.push_back(std::make_pair("is_en_passant", LogicalType::BOOLEAN));
	move_children_ext.push_back(std::make_pair("is_check", LogicalType::BOOLEAN));
	move_children_ext.push_back(std::make_pair("is_checkmate", LogicalType::BOOLEAN));
	move_children_ext.push_back(std::make_pair("is_stalemate", LogicalType::BOOLEAN));
	move_children_ext.push_back(std::make_pair("legal_response_move_count", LogicalType::UTINYINT));

	child_list_t<LogicalType> move_children;
	move_children.push_back(std::make_pair("ply", LogicalType::USMALLINT));
	move_children.push_back(std::make_pair("role", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("from", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("to", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("promotion", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("capture", LogicalType::VARCHAR));
	move_children.push_back(std::make_pair("is_castle", LogicalType::BOOLEAN));
	move_children.push_back(std::make_pair("is_en_passant", LogicalType::BOOLEAN));
	move_children.push_back(std::make_pair("is_check", LogicalType::BOOLEAN));
	move_children.push_back(std::make_pair("is_checkmate", LogicalType::BOOLEAN));

	auto move_details_ext_function = ScalarFunction(
	    "move_details_ext", {}, LogicalType::LIST(LogicalType::STRUCT(move_children_ext)), MoveDetailsExtFn);
	move_details_ext_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	move_details_ext_function.SetFallible();
	RegisterFunc(loader, move_details_ext_function, "Returns a list of extended details of all moves in the game.",
	             "move_details_ext('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");

	auto move_details_ext_at_function =
	    ScalarFunction("move_details_ext_at", {}, LogicalType::STRUCT(move_children_ext), MoveDetailsExtAtFn);
	move_details_ext_at_function.GetSignature()
	    .AddParameter("movedata", LogicalType::BLOB)
	    .AddParameter("position", LogicalType::SMALLINT);
	move_details_ext_at_function.SetFallible();
	RegisterFunc(loader, move_details_ext_at_function,
	             "Returns the extended details of a given move in the game. The move is 0-indexed, and negative "
	             "indices are allowed, though positive indices is typically faster.",
	             "move_details_ext('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");

	auto move_details_function =
	    ScalarFunction("move_details", {}, LogicalType::LIST(LogicalType::STRUCT(move_children)), MoveDetailsFn);
	move_details_function.GetSignature().AddParameter("movedata", LogicalType::BLOB);
	move_details_function.SetFallible();
	RegisterFunc(loader, move_details_function, "Returns a list of details of all moves in the game.",
	             "move_details('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");

	auto move_details_at_function =
	    ScalarFunction("move_details_at", {}, LogicalType::STRUCT(move_children), MoveDetailsAtFn);
	move_details_at_function.GetSignature()
	    .AddParameter("movedata", LogicalType::BLOB)
	    .AddParameter("position", LogicalType::SMALLINT);
	move_details_at_function.SetFallible();
	RegisterFunc(loader, move_details_at_function,
	             "Returns the details of a given move in the game. The move is 0-indexed, and negative "
	             "indices are allowed, though positive indices is typically faster.",
	             "move_details_ext('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB)");
}

} // namespace duckdb