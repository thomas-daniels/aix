#pragma once

#include "duckdb.hpp"
#include "duckdb/common/vector_operations/generic_executor.hpp"
#include "duckdb/planner/expression/bound_function_expression.hpp"
#include "duckdb/parser/parsed_data/create_scalar_function_info.hpp"

#include "rust/Game.hpp"
#include "rust/Bitboards.hpp"
#include "rust/Fen.hpp"
#include "rust/Subfen.hpp"
#include "rust/ScoutfishQuery.hpp"
#include "rust/ScoutfishQueryParseError.hpp"
#include "rust/MoveDetails.hpp"
#include "rust/MoveDetailsExtended.hpp"
#include "rust/MoveDetailsIterator.hpp"
#include "rust/MoveDetailsExtIterator.hpp"
#include "rust/DecodeError.hpp"
#include "rust/diplomat_runtime.hpp"

#include "bits.h"

namespace duckdb {

void Register_FenAtPosition(ExtensionLoader &loader);
void Register_PiecesAtPosition(ExtensionLoader &loader);
void Register_PieceCountsAtPosition(ExtensionLoader &loader);
void Register_BoardAtPosition(ExtensionLoader &loader);
void Register_MatchesSubfen(ExtensionLoader &loader);
void Register_MatchesFen(ExtensionLoader &loader);
void Register_ScoutfishQuery(ExtensionLoader &loader);
void Register_ClocksToMoveTimes(ExtensionLoader &loader);
void Register_LichessTimeControl(ExtensionLoader &loader);
void Register_LichessWinningChances(ExtensionLoader &loader);
void Register_EvalConversions(ExtensionLoader &loader);
void Register_ToUci(ExtensionLoader &loader);
void Register_ToPgn(ExtensionLoader &loader);
void Register_MovedPieces(ExtensionLoader &loader);
void Register_MoveDetails(ExtensionLoader &loader);
void Register_Recompress(ExtensionLoader &loader);
void Register_IsValidMovedata(ExtensionLoader &loader);

template <typename T>
T UnwrapDecoded(diplomat::result<T, DecodeError> &&result, const char *function_name) {
	if (result.is_ok()) {
		return *(std::move(result).ok());
	} else {
		const auto err = std::move(result).err().value();
		const auto fn_name = std::string(function_name);
		const auto code = std::to_string(static_cast<int>(err));
		throw InvalidInputException(fn_name + " - failed to decode movedata (error code " + code + ")");
	}
}

template <typename T>
std::optional<T> UnwrapOptionalDecoded(diplomat::result<T, DecodeError> &&result, const char *function_name) {
	if (result.is_ok()) {
		return std::move(result).ok();
	} else {
		const auto err = *std::move(result).err();
		const auto code = static_cast<int>(err);

		if (code == 0) {
			return std::nullopt;
		}

		const auto fn_name = std::string(function_name);
		const auto code_s = std::to_string(code);
		throw InvalidInputException(fn_name + " - failed to decode movedata (error code " + code_s + ")");
	}
}

inline void RegisterFunc(ExtensionLoader &loader, ScalarFunction fn, std::string description, std::string example) {
	CreateScalarFunctionInfo info(fn);
	FunctionDescription desc;
	desc.description = description;
	desc.examples = {example};
	info.descriptions.push_back(desc);
	loader.RegisterFunction(std::move(info));
}

inline diplomat::span<const uint8_t> game_to_span(const string_t &game) {
	return {const_data_ptr_cast(game.GetData()), static_cast<size_t>(game.GetSize())};
}

} // namespace duckdb
