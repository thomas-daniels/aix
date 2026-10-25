#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

struct MatchesFenBindData : public FunctionData {
	explicit MatchesFenBindData(Fen fen_p, bool is_null) : fen(std::move(fen_p)), is_null(is_null) {
	}

	Fen fen;
	bool is_null;

	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<MatchesFenBindData>(fen, is_null);
	}

	bool Equals(const FunctionData &other_p) const override {
		auto &other = other_p.Cast<MatchesFenBindData>();
		return fen.white == other.fen.white && fen.black == other.fen.black && fen.king == other.fen.king &&
		       fen.queen == other.fen.queen && fen.rook == other.fen.rook && fen.bishop == other.fen.bishop &&
		       fen.knight == other.fen.knight && fen.pawn == other.fen.pawn &&
		       fen.white_to_move == other.fen.white_to_move && fen.castling_rights == other.fen.castling_rights &&
		       fen.ep_square == other.fen.ep_square;
	}
};

static unique_ptr<FunctionData> MatchesFenBindFunction(BindScalarFunctionInput &input) {
	Value options_str = input.GetConstant(1, true);
	auto fen_string = options_str.GetValue<string>();
	Fen fen;
	bool is_null = options_str.IsNull();
	if (!is_null) {
		auto fen_result = Fen::parse(fen_string);
		if (fen_result.is_err()) {
			throw InvalidInputException(*input.GetArguments()[1], "failed to parse fen");
		}
		fen = *std::move(fen_result).ok();
	}
	return make_uniq<MatchesFenBindData>(fen, is_null);
}

inline void MatchesFen(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &func_expr = state.expr.Cast<BoundFunctionExpression>();
	auto &info = func_expr.BindInfo()->Cast<MatchesFenBindData>();

	if (info.is_null) {
		result.SetVectorType(VectorType::CONSTANT_VECTOR);
		ConstantVector::SetNull(result, true);
		return;
	}

	auto &game_vector = args.data[0];
	auto count = args.size();

	auto fen = info.fen;

	UnaryExecutor::Execute<string_t, bool>(game_vector, result, count, [&](string_t game) {
		diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		return UnwrapDecoded<bool>(fen.matches_fen(data), "matches_fen");
	});
}

} // namespace

void Register_MatchesFen(ExtensionLoader &loader) {
	auto matches_fen_function =
	    ScalarFunction("matches_fen", {}, LogicalType::BOOLEAN, MatchesFen, MatchesFenBindFunction);
	matches_fen_function.GetSignature()
	    .AddParameter("movedata", LogicalType::BLOB)
	    .AddParameter("fen", LogicalType::VARCHAR);
	matches_fen_function.SetFallible();
	RegisterFunc(
	    loader, matches_fen_function,
	    "Returns true if any position in the game exactly matches a given the piece placement, castling rights, side "
	    "to move, and en passant square of a given FEN. The move counters are ignored. Requires an exact board match.",
	    "matches_fen('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB, "
		"'r1bqkbnr/p1pp1ppp/1pn5/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq -')");
}

} // namespace duckdb
