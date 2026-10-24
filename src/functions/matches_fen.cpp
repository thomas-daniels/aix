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
	auto matches_fen_function = ScalarFunction("matches_fen", {LogicalType::BLOB, LogicalType::VARCHAR},
	                                           LogicalType::BOOLEAN, MatchesFen, MatchesFenBindFunction);
	matches_fen_function.SetFallible();
	loader.RegisterFunction(matches_fen_function);
}

} // namespace duckdb
