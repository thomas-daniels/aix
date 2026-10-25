#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

struct MatchesSubfenBindData : public FunctionData {
	explicit MatchesSubfenBindData(Subfen subfen_p, bool is_null) : subfen(std::move(subfen_p)), is_null(is_null) {
	}

	Subfen subfen;
	bool is_null;

	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<MatchesSubfenBindData>(subfen, is_null);
	}

	bool Equals(const FunctionData &other_p) const override {
		auto &other = other_p.Cast<MatchesSubfenBindData>();
		return subfen.white == other.subfen.white && subfen.black == other.subfen.black &&
		       subfen.king == other.subfen.king && subfen.queen == other.subfen.queen &&
		       subfen.rook == other.subfen.rook && subfen.bishop == other.subfen.bishop &&
		       subfen.knight == other.subfen.knight && subfen.pawn == other.subfen.pawn;
	}
};

static unique_ptr<FunctionData> MatchesSubfenBindFunction(BindScalarFunctionInput &input) {
	Value options_str = input.GetConstant(1, true);
	auto subfen_string = options_str.GetValue<string>();
	Subfen subfen;
	bool is_null = options_str.IsNull();
	if (!is_null) {
		auto subfen_result = Subfen::parse(subfen_string);
		if (subfen_result.is_err()) {
			throw InvalidInputException(*input.GetArguments()[1], "failed to parse subfen");
		}
		subfen = *std::move(subfen_result).ok();
	}
	return make_uniq<MatchesSubfenBindData>(subfen, is_null);
}

inline void MatchesSubfen(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &func_expr = state.expr.Cast<BoundFunctionExpression>();
	auto &info = func_expr.BindInfo()->Cast<MatchesSubfenBindData>();

	if (info.is_null) {
		result.SetVectorType(VectorType::CONSTANT_VECTOR);
		ConstantVector::SetNull(result, true);
		return;
	}

	auto &game_vector = args.data[0];
	auto count = args.size();

	auto subfen = info.subfen;

	UnaryExecutor::Execute<string_t, bool>(game_vector, result, count, [&](string_t game) {
		diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
		return UnwrapDecoded<bool>(subfen.matches(data), "matches_subfen");
	});
}

} // namespace

void Register_MatchesSubfen(ExtensionLoader &loader) {
	auto matches_subfen_function =
	    ScalarFunction("matches_subfen", {}, LogicalType::BOOLEAN, MatchesSubfen, MatchesSubfenBindFunction);
	matches_subfen_function.GetSignature()
	    .AddParameter("movedata", LogicalType::BLOB)
	    .AddParameter("fen", LogicalType::VARCHAR);
	matches_subfen_function.SetFallible();
	RegisterFunc(loader, matches_subfen_function,
	             "Returns true if any position in the game matches a given sub-FEN. A sub-FEN consists of only the "
	             "piece placement part of a FEN and matches if a position contains at least those pieces.",
	             "matches_subfen('\\x9F-\\x84\\x1C\\x1A\\x9D:\\xBD\\xB3\\xB8'::BLOB, "
	             "'r1bq2kr/p1pp2pp/1pn2n2/2b1p1N1/4P3/1QP5/PP1P1PPP/RNB2RK1')");
}

} // namespace duckdb