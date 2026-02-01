#include "aixchess_functions.hpp"

namespace duckdb {

namespace {

using STR = PrimitiveType<string_t>;
using STR_LIST = GenericListType<PrimitiveType<string_t>>;

const char *SQUARES[] = {"a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2",
                         "f2", "g2", "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4",
                         "c4", "d4", "e4", "f4", "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5",
                         "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a7", "b7", "c7", "d7",
                         "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"};

struct PiecesAtPositionResult {
	bool valid;

	STR wk_val;
	STR_LIST wq_val;
	STR_LIST wr_val;
	STR_LIST wb_val;
	STR_LIST wn_val;
	STR_LIST wp_val;
	STR bk_val;
	STR_LIST bq_val;
	STR_LIST br_val;
	STR_LIST bb_val;
	STR_LIST bn_val;
	STR_LIST bp_val;

	static void AssignResult(Vector &result, idx_t i, PiecesAtPositionResult value) {
		if (!value.valid) {
			FlatVector::SetNull(result, i, true);
			return;
		}

		// no need for a StringVector::AddString call here, because the strings are so short that they will always
		// be inlined

		auto &entries = StructVector::GetEntries(result);
		STR::AssignResult(*entries[0], i, value.wk_val);
		STR_LIST::AssignResult(*entries[1], i, value.wq_val);
		STR_LIST::AssignResult(*entries[2], i, value.wr_val);
		STR_LIST::AssignResult(*entries[3], i, value.wb_val);
		STR_LIST::AssignResult(*entries[4], i, value.wn_val);
		STR_LIST::AssignResult(*entries[5], i, value.wp_val);
		STR::AssignResult(*entries[6], i, value.bk_val);
		STR_LIST::AssignResult(*entries[7], i, value.bq_val);
		STR_LIST::AssignResult(*entries[8], i, value.br_val);
		STR_LIST::AssignResult(*entries[9], i, value.bb_val);
		STR_LIST::AssignResult(*entries[10], i, value.bn_val);
		STR_LIST::AssignResult(*entries[11], i, value.bp_val);
	}
};

void BitboardToSquareList(uint64_t bb, STR_LIST &out) {
	for (idx_t i = 0; i < 8; i++) {
		for (idx_t j = 0; j < 8; j++) {
			idx_t sq = i + j * 8; // this makes sure the squares are ordered lexicographically
			if (bb & (uint64_t(1) << sq)) {
				out.values.push_back(STR(SQUARES[sq]));
			}
		}
	}
}

const char *BitboardToSquare(uint64_t bb) {
	if (bb == 0) {
		return "";
	}
	idx_t sq = bits::countr_zero(bb);
	return SQUARES[sq];
}

inline void PiecesAtPosition(DataChunk &args, ExpressionState &state, Vector &result) {
	GenericExecutor::ExecuteBinary<PrimitiveType<string_t>, PrimitiveType<int32_t>, PiecesAtPositionResult>(
	    args.data[0], args.data[1], result, args.size(), [&](PrimitiveType<string_t> game, PrimitiveType<int32_t> pos) {
		    diplomat::span<const uint8_t> data = {const_data_ptr_cast(game.val.GetData()), game.val.GetSize()};
		    auto bitboards_result = Game::pieces_at_position(data, pos.val);
		    const auto bitboards_opt =
		        UnwrapOptionalDecoded<Bitboards>(std::move(bitboards_result), "pieces_at_position");

		    PiecesAtPositionResult res;

		    if (!bitboards_opt.has_value()) {
			    res.valid = false;
			    return res;
		    }

		    res.valid = true;

		    const Bitboards &bitboards = *bitboards_opt;

		    res.wk_val = STR(BitboardToSquare(bitboards.w_k));
		    BitboardToSquareList(bitboards.w_q, res.wq_val);
		    BitboardToSquareList(bitboards.w_r, res.wr_val);
		    BitboardToSquareList(bitboards.w_b, res.wb_val);
		    BitboardToSquareList(bitboards.w_n, res.wn_val);
		    BitboardToSquareList(bitboards.w_p, res.wp_val);
		    res.bk_val = STR(BitboardToSquare(bitboards.b_k));
		    BitboardToSquareList(bitboards.b_q, res.bq_val);
		    BitboardToSquareList(bitboards.b_r, res.br_val);
		    BitboardToSquareList(bitboards.b_b, res.bb_val);
		    BitboardToSquareList(bitboards.b_n, res.bn_val);
		    BitboardToSquareList(bitboards.b_p, res.bp_val);
		    return res;
	    });
}

} // namespace

void Register_PiecesAtPosition(ExtensionLoader &loader) {
	child_list_t<LogicalType> piece_counts_children;
	piece_counts_children.push_back(std::make_pair("wK", LogicalType::VARCHAR));
	piece_counts_children.push_back(std::make_pair("wQ", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("wR", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("wB", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("wN", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("wP", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("bK", LogicalType::VARCHAR));
	piece_counts_children.push_back(std::make_pair("bQ", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("bR", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("bB", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("bN", LogicalType::LIST(LogicalType::VARCHAR)));
	piece_counts_children.push_back(std::make_pair("bP", LogicalType::LIST(LogicalType::VARCHAR)));

	auto pieces_pos_function = ScalarFunction("pieces_at_position", {LogicalType::BLOB, LogicalType::INTEGER},
	                                          LogicalType::STRUCT(piece_counts_children), PiecesAtPosition);
	loader.RegisterFunction(pieces_pos_function);
}

} // namespace duckdb