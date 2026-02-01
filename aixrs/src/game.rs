use super::{decode_bytes, decode_bytes_positions};
use crate::board_into_bitboards;
use crate::ffi::{Bitboards, Game, MoveDetails};
use aix_chess_compression::{Decode, Decoder, EncodedGame};
use diplomat_runtime::DiplomatWrite;
use shakmaty::fen::Fen;
use shakmaty::{Chess, Color, EnPassantMode, Position, Setup};
use std::fmt::Write;

pub fn pieces_at_position(data: &[u8], pos: i32) -> Result<Bitboards, crate::ffi::DecodeError> {
    let positions = decode_bytes_positions(data)?;

    let index = if pos >= 0 {
        pos
    } else {
        positions.len() as i32 + pos + 1
    };

    if index < 0 || index as usize > positions.len() {
        Err(crate::ffi::DecodeError::NoErrorNoValue)
    } else {
        Ok(if index == 0 {
            board_into_bitboards(&Chess::new().board())
        } else {
            board_into_bitboards(positions[index as usize - 1].board())
        })
    }
}

pub fn board_at_position(
    data: &[u8],
    pos: i32,
    out: &mut [i8],
) -> Result<(), crate::ffi::DecodeError> {
    let positions_result = decode_bytes(data).map(|(_, p)| p);

    positions_result.and_then(|positions| {
        let index = if pos >= 0 {
            pos
        } else {
            positions.len() as i32 + pos + 1
        };

        if index < 0 || index as usize > positions.len() {
            Err(crate::ffi::DecodeError::NoErrorNoValue)
        } else {
            let setup = if index == 0 {
                Setup::default()
            } else {
                positions[index as usize - 1]
                    .clone()
                    .to_setup(EnPassantMode::Always)
            };

            for (sq, p) in setup.board {
                out[sq as usize] = p.char() as i8;
            }

            Ok(())
        }
    })
}

pub fn fen_at_position(
    data: &[u8],
    pos: i32,
    out: &mut DiplomatWrite,
) -> Result<(), crate::ffi::DecodeError> {
    let positions_result = decode_bytes(data).map(|(_, p)| p);

    positions_result.and_then(|positions| {
        let index = if pos >= 0 {
            pos
        } else {
            positions.len() as i32 + pos + 1
        };

        if index < 0 || index as usize > positions.len() {
            Err(crate::ffi::DecodeError::NoErrorNoValue)
        } else {
            let fen = if index == 0 {
                let pos = Chess::default();
                Fen::from_position(&pos, EnPassantMode::Always)
            } else {
                Fen::from_position(&positions[index as usize - 1], EnPassantMode::Always)
            }
            .to_string();
            write!(out, "{fen}").expect("fen_at_position: write to DiplomatWrite failed");
            Ok(())
        }
    })
}

pub fn to_uci_string(data: &[u8], out: &mut DiplomatWrite) -> Result<(), crate::ffi::DecodeError> {
    let encoded = EncodedGame::from_bytes(data)?;
    let decoder = Decoder::new(&encoded);
    let uci_string = decoder.into_uci_string()?;
    write!(out, "{uci_string}").unwrap();
    Ok(())
}

pub fn to_pgn_string(data: &[u8], out: &mut DiplomatWrite) -> Result<(), crate::ffi::DecodeError> {
    let encoded = EncodedGame::from_bytes(data)?;
    let decoder = Decoder::new(&encoded);
    let pgn_string = decoder.into_pgn_string()?;
    write!(out, "{pgn_string}").unwrap();
    Ok(())
}

pub fn moved_pieces(data: &[u8], out: &mut DiplomatWrite) -> Result<(), crate::ffi::DecodeError> {
    let encoded = EncodedGame::from_bytes(data)?;
    let decoder = Decoder::new(&encoded);
    for (i, m) in decoder.into_iter_moves().enumerate() {
        let piece = m?
            .role()
            .of(if i % 2 == 0 {
                Color::White
            } else {
                Color::Black
            })
            .char();
        write!(out, "{piece}").unwrap();
    }

    Ok(())
}

pub fn from_bytes(data: &'_ [u8]) -> Result<Box<Game<'_>>, crate::ffi::DecodeError> {
    Ok(Box::new(Game(EncodedGame::from_bytes(data)?)))
}

fn castling_king_dest(king: shakmaty::Square, rook: shakmaty::Square) -> shakmaty::Square {
    let side = shakmaty::CastlingSide::from_king_side(king < rook);
    shakmaty::Square::from_coords(side.king_to_file(), king.rank())
}

pub fn move_details_iterator<'a>(
    encoded: &'a EncodedGame,
) -> impl Iterator<Item = Result<MoveDetails, crate::ffi::DecodeError>> + 'a {
    let decoder = Decoder::new(encoded);
    decoder
        .into_iter_moves_and_positions()
        .enumerate()
        .map(|(ply, r)| {
            r.map(|(m, pos)| {
                let from = m.from().expect("from() should always be Some(...)") as u8;
                let to = match m {
                    shakmaty::Move::Normal { to, .. }
                    | shakmaty::Move::EnPassant { to, .. }
                    | shakmaty::Move::Put { to, .. } => to,
                    shakmaty::Move::Castle { king, rook } => castling_king_dest(king, rook),
                } as u8;
                let capture = match m.capture() {
                    Some(role) => role.char() as i8,
                    None => 0,
                };
                let is_castle = m.is_castle();
                let promotion = match m.promotion() {
                    Some(role) => role.char() as i8,
                    None => 0,
                };
                let role = m.role().char() as i8;
                let ply = ply as u16;

                let is_check = pos.is_check();
                let is_checkmate = pos.is_checkmate();

                let is_en_passant = m.is_en_passant();

                MoveDetails {
                    ply,
                    role,
                    from,
                    to,
                    capture,
                    is_castle,
                    promotion,
                    is_check,
                    is_checkmate,
                    is_en_passant,
                }
            })
            .map_err(|e| e.into())
        })
}
