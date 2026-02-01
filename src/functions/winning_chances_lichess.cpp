#include "aixchess_functions.hpp"

#include <algorithm>
#include <cmath>
#include <climits>

namespace duckdb {

namespace {

double RawWinningChances(int16_t eval) {
	const double multiplier = -0.00368208;
	return 2 / (1 + exp(multiplier * eval)) - 1;
}

inline void LichessWinningChances(DataChunk &args, ExpressionState &state, Vector &result) {
	UnaryExecutor::Execute<int16_t, double_t>(args.data[0], result, args.size(), [&](int16_t eval) {
		int16_t transformed_eval;
		if (eval >= SHRT_MAX - 511) {
			int16_t mate = SHRT_MAX - eval + 1;
			transformed_eval = (21 - std::min(mate, int16_t(10))) * 100;
		} else if (eval >= 0) {
			transformed_eval = std::min(eval, int16_t(1000));
		} else if (eval > SHRT_MIN + 511) {
			transformed_eval = std::max(eval, int16_t(-1000));
		} else {
			int16_t mate = -(SHRT_MIN - eval) + 1;
			transformed_eval = -(21 - std::min(mate, int16_t(10))) * 100;
		}
		return RawWinningChances(transformed_eval);
	});
}

} // namespace

void Register_LichessWinningChances(ExtensionLoader &loader) {
	auto lichess_winning_chances_function =
	    ScalarFunction("winning_chances_lichess", {LogicalType::SMALLINT}, LogicalType::DOUBLE, LichessWinningChances);
	loader.RegisterFunction(lichess_winning_chances_function);
}

} // namespace duckdb