#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

struct ScoutfishQueryBindData : public FunctionData {
	explicit ScoutfishQueryBindData(std::vector<uint8_t> encoded_query_p, bool is_null)
	    : encoded_query(std::move(encoded_query_p)), is_null(is_null) {
	}

	std::vector<uint8_t> encoded_query;
	bool is_null;

	unique_ptr<FunctionData> Copy() const override {
		return make_uniq<ScoutfishQueryBindData>(encoded_query, is_null);
	}

	bool Equals(const FunctionData &other_p) const override {
		auto &other = other_p.Cast<ScoutfishQueryBindData>();
		return encoded_query == other.encoded_query;
	}
};

static const char *ScoutfishParseErrorToString(ScoutfishQueryParseError err) {
	switch (err) {
	case ScoutfishQueryParseError::InvalidPiece:
		return "Scoutfish query parsing error: invalid piece in query";
	case ScoutfishQueryParseError::InvalidImbalanceFormat:
		return "Scoutfish query parsing error: invalid format for 'imbalance'";
	case ScoutfishQueryParseError::InvalidMaterialFormat:
		return "Scoutfish query parsing error: invalid format for 'material'";
	case ScoutfishQueryParseError::InvalidSideToMove:
		return "Scoutfish query parsing error: invalid side to move";
	case ScoutfishQueryParseError::InvalidSan:
		return "Scoutfish query parsing error: invalid SAN in white-move or black-move";
	case ScoutfishQueryParseError::InvalidSyntaxOrStructure:
		return "Scoutfish query parsing error: invalid query syntax or structure";
	case ScoutfishQueryParseError::BincodeError:
		return "Scoutfish query parsing internal error (please report): BincodeError";
	case ScoutfishQueryParseError::BufferTooSmall:
		return "Scoutfish query parsing internal error (please report): BufferTooSmall";
	case ScoutfishQueryParseError::CursorWriteError:
		return "Scoutfish query parsing internal error (please report): CursorWriteError";
	default:
		return "Scoutfish query parsing internal error (please report): Unknown error";
	}
}

static unique_ptr<FunctionData> ScoutfishQueryBindFunction(ClientContext &context, ScalarFunction &bound_function,
                                                           vector<unique_ptr<Expression>> &arguments) {
	auto &query_arg = arguments[1];
	if (query_arg->HasParameter()) {
		throw ParameterNotResolvedException();
	}
	if (!query_arg->IsFoldable()) {
		throw InvalidInputException(*query_arg, "Scoutfish query must be a constant");
	}
	Value options_str = ExpressionExecutor::EvaluateScalar(context, *query_arg);
	auto scoutfish_query_string = options_str.GetValue<string>();

	bool is_null = options_str.IsNull();

	if (!is_null) {
		std::vector<uint8_t> encoded_query(128 + scoutfish_query_string.size() * 4);
		auto encoded_query_span = diplomat::span<uint8_t>(encoded_query.data(), encoded_query.size());
		auto res = ScoutfishQuery::parse_into_bytes(scoutfish_query_string, encoded_query_span);
		if (res.is_err()) {
			auto err = std::move(res).err().value();
			throw InvalidInputException(*query_arg, ScoutfishParseErrorToString(err));
		}
		auto size = std::move(res).ok().value();
		encoded_query.resize(size);
		return make_uniq<ScoutfishQueryBindData>(encoded_query, is_null);
	} else {
		std::vector<uint8_t> encoded_query(0);
		return make_uniq<ScoutfishQueryBindData>(encoded_query, is_null);
	}
}

template <bool PLIES>
inline void ScoutfishQuery(DataChunk &args, ExpressionState &state, Vector &result) {
	auto &func_expr = state.expr.Cast<BoundFunctionExpression>();
	auto &info = func_expr.bind_info->Cast<ScoutfishQueryBindData>();

	if (info.is_null) {
		result.SetVectorType(VectorType::CONSTANT_VECTOR);
		ConstantVector::SetNull(result, true);
		return;
	}

	auto &game_vector = args.data[0];
	auto count = args.size();

	auto encoded_query_span = diplomat::span<const uint8_t>(info.encoded_query.data(), info.encoded_query.size());
	auto query_r = ScoutfishQuery::decode_bytes(encoded_query_span);
	if (query_r.is_err()) {
		throw InvalidInputException("Scoutfish query internal error (please report): decode_bytes");
	}
	auto query = std::move(query_r).ok().value();

	if constexpr (!PLIES) {
		UnaryExecutor::Execute<string_t, bool>(game_vector, result, count, [&](string_t game) {
			diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.GetData()), game.GetSize()};
			return UnwrapDecoded(query->matches(data), "scoutfish_query");
		});
	} else {
		GenericExecutor::ExecuteUnary<PrimitiveType<string_t>, GenericListType<PrimitiveType<uint16_t>>>(
		    game_vector, result, count, [&](PrimitiveType<string_t> game) {
			    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};
			    const auto plies_data_size = 16;
			    uint32_t plies_data[plies_data_size] = {0};
			    const auto query_result =
			        UnwrapDecoded(query->matches_plies(data, {plies_data, 16}), "scoutfish_query_plies");

			    const auto len = query_result & 0b1111111111111111;
			    const auto min = static_cast<uint16_t>(query_result >> 16);

			    GenericListType<PrimitiveType<uint16_t>> plies_list;
			    auto added = 0;
			    for (auto i = 0; i < plies_data_size; i++) {
				    auto plies_data_curr = plies_data[i];
				    if (plies_data_curr == 0) {
					    continue;
				    }

#if __cplusplus >= 201907L
				    auto start = std::countr_zero(plies_data_curr);
				    auto end = 32 - std::countl_zero(plies_data_curr);
#else
					auto start = 0;
					auto end = 32;
#endif

				    for (auto j = start; j < end; j++) {
					    if (plies_data[i] & (1u << j)) {
						    auto ply = static_cast<uint16_t>(i * 32 + j) + min;
						    plies_list.values.push_back(PrimitiveType<uint16_t>(ply));
						    added++;
					    }
				    }
				    if (added >= len) {
					    break;
				    }
			    }
			    return plies_list;
		    });
	}
}

} // namespace

void Register_ScoutfishQuery(ExtensionLoader &loader) {
	auto scoutfish_query_function =
	    ScalarFunction("scoutfish_query", {LogicalType::BLOB, LogicalType::VARCHAR}, LogicalType::BOOLEAN,
	                   ScoutfishQuery<false>, ScoutfishQueryBindFunction);
	loader.RegisterFunction(scoutfish_query_function);

	auto scoutfish_query_plies_function =
	    ScalarFunction("scoutfish_query_plies", {LogicalType::BLOB, LogicalType::VARCHAR},
	                   LogicalType::LIST(LogicalType::USMALLINT), ScoutfishQuery<true>, ScoutfishQueryBindFunction);
	loader.RegisterFunction(scoutfish_query_plies_function);
}

} // namespace duckdb