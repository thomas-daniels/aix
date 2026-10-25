#include "aixchess_functions.hpp"

#include <algorithm>
#include <cmath>
#include <climits>

namespace duckdb {

namespace {

inline void EvalToCentipawns(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<int16_t, int16_t>(args.data[0], result, args.size(),
	                                         [&](int16_t eval) -> std::optional<int16_t> {
		                                         if (eval >= SHRT_MAX - 511 || eval <= SHRT_MIN + 511) {
			                                         return std::nullopt;
		                                         } else {
			                                         return eval;
		                                         }
	                                         });
}

inline void EvalToMate(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<int16_t, int16_t>(args.data[0], result, args.size(),
	                                         [&](int16_t eval) -> std::optional<int16_t> {
		                                         if (eval >= SHRT_MAX - 511) {
			                                         return static_cast<int16_t>(SHRT_MAX - eval + 1);
		                                         } else if (eval <= SHRT_MIN + 511) {
			                                         return static_cast<int16_t>(SHRT_MIN - eval - 1);
		                                         } else {
			                                         return std::nullopt;
		                                         }
	                                         });
}

} // namespace

void Register_EvalConversions(ExtensionLoader &loader) {
	auto eval_to_centipawns_function =
	    ScalarFunction("eval_to_centipawns", {}, LogicalType::SMALLINT, EvalToCentipawns);
	eval_to_centipawns_function.GetSignature().AddParameter("eval", LogicalType::SMALLINT);
	RegisterFunc(loader, eval_to_centipawns_function,
	             "Convert representation of engine eval to centipawns, or NULL if it represents mate.",
	             "eval_to_centipawns(100)");

	auto eval_to_mate_function = ScalarFunction("eval_to_mate", {}, LogicalType::SMALLINT, EvalToMate);
	eval_to_mate_function.GetSignature().AddParameter("eval", LogicalType::SMALLINT);
	RegisterFunc(
	    loader, eval_to_mate_function,
	    "Convert representation of engine eval to mate-in-N, or NULL if it represents a (dis)advantage in centipawns.",
	    "eval_to_mate(-32768)");
}

} // namespace duckdb