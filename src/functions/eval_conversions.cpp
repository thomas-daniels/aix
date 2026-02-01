#include "aixchess_functions.hpp"

#include <algorithm>
#include <cmath>
#include <climits>

namespace duckdb {

namespace {

inline void EvalToCentipawns(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::ExecuteWithNulls<int16_t, int16_t>(args.data[0], result, args.size(),
	                                                  [&](int16_t eval, ValidityMask &mask, idx_t idx) {
		                                                  if (eval >= SHRT_MAX - 511 || eval <= SHRT_MIN + 511) {
			                                                  mask.SetInvalid(idx);
			                                                  return int16_t(0);
		                                                  } else {
			                                                  return eval;
		                                                  }
	                                                  });
}

inline void EvalToMate(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::ExecuteWithNulls<int16_t, int16_t>(args.data[0], result, args.size(),
	                                                  [&](int16_t eval, ValidityMask &mask, idx_t idx) {
		                                                  if (eval >= SHRT_MAX - 511) {
			                                                  return static_cast<int16_t>(SHRT_MAX - eval + 1);
		                                                  } else if (eval <= SHRT_MIN + 511) {
			                                                  return static_cast<int16_t>(SHRT_MIN - eval - 1);
		                                                  } else {
			                                                  mask.SetInvalid(idx);
			                                                  return int16_t(0);
		                                                  }
	                                                  });
}

} // namespace

void Register_EvalConversions(ExtensionLoader &loader) {
	auto eval_to_centipawns_function =
	    ScalarFunction("eval_to_centipawns", {LogicalType::SMALLINT}, LogicalType::SMALLINT, EvalToCentipawns);
	loader.RegisterFunction(eval_to_centipawns_function);

	auto eval_to_mate_function =
	    ScalarFunction("eval_to_mate", {LogicalType::SMALLINT}, LogicalType::SMALLINT, EvalToMate);
	loader.RegisterFunction(eval_to_mate_function);
}

} // namespace duckdb