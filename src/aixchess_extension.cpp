#define DUCKDB_EXTENSION_MAIN

#include "aixchess_extension.hpp"
#include "aixchess_functions.hpp"
#include "duckdb.hpp"
#include "duckdb/common/exception.hpp"
#include "duckdb/function/scalar_function.hpp"
#include <duckdb/parser/parsed_data/create_scalar_function_info.hpp>
#include "duckdb/catalog/default/default_functions.hpp"

namespace duckdb {

static DefaultMacro aixchess_macros[] = {{DEFAULT_SCHEMA,
                                          "list_winning_chances_lichess",
                                          {"evals", nullptr},
                                          {{nullptr, nullptr}},
                                          R"( list_transform(evals, lambda x: winning_chances_lichess(x)) )"},
                                         {DEFAULT_SCHEMA,
                                          "list_eval_to_centipawns",
                                          {"evals", nullptr},
                                          {{nullptr, nullptr}},
                                          R"( list_transform(evals, lambda x: eval_to_centipawns(x)) )"},
                                         {DEFAULT_SCHEMA,
                                          "list_eval_to_mate",
                                          {"evals", nullptr},
                                          {{nullptr, nullptr}},
                                          R"( list_transform(evals, lambda x: eval_to_mate(x)) )"},
                                         {nullptr, nullptr, {nullptr}, {{nullptr, nullptr}}, nullptr}};

static void LoadInternal(ExtensionLoader &loader) {
	Register_FenAtPosition(loader);
	Register_PiecesAtPosition(loader);
	Register_PieceCountsAtPosition(loader);
	Register_BoardAtPosition(loader);
	Register_MatchesSubfen(loader);
	Register_ScoutfishQuery(loader);
	Register_ClocksToMoveTimes(loader);
	Register_LichessTimeControl(loader);
	Register_LichessWinningChances(loader);
	Register_EvalConversions(loader);
	Register_ToUci(loader);
	Register_ToPgn(loader);
	Register_MovedPieces(loader);
	Register_MoveDetails(loader);
	Register_Recompress(loader);

	// Macros
	for (idx_t index = 0; aixchess_macros[index].name != nullptr; index++) {
		auto info = DefaultFunctionGenerator::CreateInternalMacroInfo(aixchess_macros[index]);
		loader.RegisterFunction(*info);
	}
}

void AixchessExtension::Load(ExtensionLoader &loader) {
	LoadInternal(loader);
}
std::string AixchessExtension::Name() {
	return "aixchess";
}

std::string AixchessExtension::Version() const {
#ifdef EXT_VERSION_AIXCHESS
	return EXT_VERSION_AIXCHESS;
#else
	return "";
#endif
}

} // namespace duckdb

extern "C" {

DUCKDB_CPP_EXTENSION_ENTRY(aixchess, loader) {
	duckdb::LoadInternal(loader);
}
}
