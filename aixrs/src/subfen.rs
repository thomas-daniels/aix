use crate::ffi::Subfen;
use aix_chess_compression::{Decode, Decoder, EncodedGame};
use shakmaty::{fen::ParseFenError, Board, Position};

pub fn try_parse(subfen: &[u8]) -> Result<Subfen, ParseFenError> {
    let board = Board::from_ascii_board_fen(subfen)?;
    Ok(Subfen {
        white: board.white().0,
        black: board.black().0,
        king: board.kings().0,
        queen: board.queens().0,
        rook: board.rooks().0,
        bishop: board.bishops().0,
        knight: board.knights().0,
        pawn: board.pawns().0,
    })
}

pub fn matches(subfen: Subfen, game: &[u8]) -> Result<bool, crate::ffi::DecodeError> {
    let encoded = EncodedGame::from_bytes(game)?;
    let decoder = Decoder::new(&encoded);
    for position in decoder.into_iter_positions() {
        let position = position?;
        let board = position.board();
        if matches_board(&subfen, board) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn matches_board(subfen: &Subfen, board: &Board) -> bool {
    (board.white().0 & subfen.white) == subfen.white
        && (board.black().0 & subfen.black) == subfen.black
        && (board.kings().0 & subfen.king) == subfen.king
        && (board.queens().0 & subfen.queen) == subfen.queen
        && (board.rooks().0 & subfen.rook) == subfen.rook
        && (board.bishops().0 & subfen.bishop) == subfen.bishop
        && (board.knights().0 & subfen.knight) == subfen.knight
        && (board.pawns().0 & subfen.pawn) == subfen.pawn
}
