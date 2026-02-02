use bitm::BitAccess;
use chess_huffman as huff;
use shakmaty::{Chess, Color, File, Move, Position, Rank, Role, Square, uci::UciMove};

use crate::{Decode, DecodeError, DecodeResult, EncodeError, EncodedGame, EncodedGameContent};

static NEEDED_BITS: [u8; 33] = [
    0, 0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5,
];

fn move_index_knight(from: Square, to: Square) -> u64 {
    let diff = to as i32 - from as i32;
    match diff {
        6 => 0,
        10 => 1,
        15 => 2,
        17 => 3,
        -6 => 4,
        -10 => 5,
        -15 => 6,
        -17 => 7,
        _ => panic!("Invalid knight move"),
    }
}

fn move_index_rook(from: Square, to: Square) -> u64 {
    if from.file() == to.file() {
        to.rank() as u64
    } else {
        // from.rank() == to.rank()
        to.file() as u64 | (1 << 3)
    }
}

fn move_index_bishop(from: Square, to: Square) -> u64 {
    let diff = to as i32 - from as i32;
    if diff % 9 == 0 {
        // Do check for % 9 first because 63 (h8 -> a1) is both divisible by 9 and 7 and we want this encoding then
        // (SW-NE)
        to.rank() as u64 | (1 << 3)
    } else {
        // diff % 7 == 0
        // (NW-SE)
        to.rank() as u64
    }
}

fn move_index_queen(from: Square, to: Square) -> u64 {
    if (from.file() == to.file()) || (from.rank() == to.rank()) {
        move_index_rook(from, to)
    } else {
        move_index_bishop(from, to) | (1 << 4)
    }
}

fn move_index_king(from: Square, to: Square, white: bool) -> u64 {
    let diff = to as i32 - from as i32;

    // For castling: if a king can castle, it must be in the initial position,
    // so we know that it cannot move back a rank and we can use those moves to
    // encode castling instead.
    match diff {
        1 => 0,
        9 => 1,
        8 => 2,
        7 => 3,
        -1 => 4,
        -9 => 5,
        -8 => 6,
        -7 => 7,
        3 => {
            if white {
                5
            } else {
                1
            }
        } // O-O
        -4 => {
            if white {
                6
            } else {
                2
            }
        } // O-O-O
        _ => panic!("Invalid king move"),
    }
}

fn move_index_pawn_standard(from: Square, to: Square) -> u64 {
    let diff = (to as i32 - from as i32).abs();
    match diff {
        8 => 0,
        7 => 1,
        9 => 2,
        16 => 3,
        _ => panic!("Invalid pawn move"),
    }
}

fn move_index_pawn_promoting(from: Square, to: Square, promotion_role: Role) -> u64 {
    (match to.file() as i32 - from.file() as i32 {
        0 => 0,
        -1 => 4,
        1 => 8,
        _ => panic!("Invalid pawn promotion move"),
    }) + match promotion_role {
        Role::Queen => 0,
        Role::Rook => 1,
        Role::Bishop => 2,
        Role::Knight => 3,
        _ => panic!("Invalid promotion piece"),
    }
}

fn move_index_and_bits(
    from: Square,
    to: Square,
    role: Role,
    white: bool,
    promotion_role: Option<Role>,
) -> (u64, u8) {
    match role {
        Role::Knight => (move_index_knight(from, to), 3),
        Role::Rook => (move_index_rook(from, to), 4),
        Role::Bishop => (move_index_bishop(from, to), 4),
        Role::Queen => (move_index_queen(from, to), 5),
        Role::King => (move_index_king(from, to, white), 3),
        Role::Pawn => match promotion_role {
            Some(promotion_role) => (move_index_pawn_promoting(from, to, promotion_role), 4),
            None => (move_index_pawn_standard(from, to), 2),
        },
    }
}

fn new_square(from: u32) -> DecodeResult<Square> {
    if from < 64 {
        Ok(unsafe { Square::new_unchecked(from) })
    } else {
        Err(DecodeError {})
    }
}

fn knight_destination_from_move_index(from: Square, move_index: u64) -> DecodeResult<Square> {
    let from = from as u32;
    new_square(match move_index {
        0 => from + 6,
        1 => from + 10,
        2 => from + 15,
        3 => from + 17,
        4 => from.wrapping_sub(6),
        5 => from.wrapping_sub(10),
        6 => from.wrapping_sub(15),
        7 => from.wrapping_sub(17),
        _ => return Err(DecodeError {}),
    })
}

#[allow(clippy::cast_possible_truncation)]
fn rook_destination_from_move_index(from: Square, move_index: u64) -> Square {
    if move_index & (1 << 3) == 0 {
        Square::from_coords(from.file(), unsafe {
            Rank::new_unchecked(move_index as u32)
        })
    } else {
        Square::from_coords(
            unsafe { File::new_unchecked(move_index as u32 & 0b111) },
            from.rank(),
        )
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn bishop_destination_from_move_index(from: Square, move_index: u64) -> DecodeResult<Square> {
    let dest_rank: u64;
    let dest_file: u32;

    if move_index & (1 << 3) == 0 {
        dest_rank = move_index;
        let rank_diff = dest_rank as i32 - from.rank() as i32;
        dest_file = (from.file() as i32 - rank_diff) as u32;
    } else {
        dest_rank = move_index & 0b111;
        let rank_diff = dest_rank as i32 - from.rank() as i32;
        dest_file = (from.file() as i32 + rank_diff) as u32;
    }

    if dest_file > 7 {
        return Err(DecodeError {});
    }

    Ok(unsafe {
        Square::from_coords(
            File::new_unchecked(dest_file),
            Rank::new_unchecked(dest_rank as u32),
        )
    })
}

fn queen_destination_from_move_index(from: Square, move_index: u64) -> DecodeResult<Square> {
    if move_index & (1 << 4) == 0 {
        Ok(rook_destination_from_move_index(from, move_index))
    } else {
        bishop_destination_from_move_index(from, move_index & 0b1111)
    }
}

fn king_destination_from_move_index(
    from: Square,
    white: bool,
    move_index: u64,
) -> DecodeResult<Square> {
    let from = from as u32;
    new_square(match move_index {
        0 => from + 1,
        1 => {
            if !white && from == 60 {
                63
            } else {
                from + 9
            }
        }
        2 => {
            if !white && from == 60 {
                56
            } else {
                from + 8
            }
        }
        3 => from + 7,
        4 => from.wrapping_sub(1),
        5 => {
            if white && from == 4 {
                7
            } else {
                from.wrapping_sub(9)
            }
        }
        6 => {
            if white && from == 4 {
                0
            } else {
                from.wrapping_sub(8)
            }
        }
        7 => from.wrapping_sub(7),
        _ => return Err(DecodeError {}),
    })
}

fn pawn_destination_from_move_index(
    from: Square,
    white: bool,
    move_index: u64,
) -> DecodeResult<(Square, Option<Role>)> {
    let from_rank = from.rank();
    let from = from as u32;

    Ok(if white {
        if from_rank == Rank::Seventh {
            (
                new_square(match move_index & 0b1100 {
                    0 => from + 8,
                    4 => from + 7,
                    8 => from + 9,
                    _ => return Err(DecodeError {}),
                })?,
                Some(match move_index & 0b11 {
                    0 => Role::Queen,
                    1 => Role::Rook,
                    2 => Role::Bishop,
                    3 => Role::Knight,
                    _ => unreachable!(),
                }),
            )
        } else {
            (
                new_square(match move_index {
                    0 => from + 8,
                    1 => from + 7,
                    2 => from + 9,
                    3 => from + 16,
                    _ => return Err(DecodeError {}),
                })?,
                None,
            )
        }
    } else if from_rank == Rank::Second {
        (
            new_square(match move_index & 0b1100 {
                0 => from - 8,
                4 => from - 9,
                8 => from - 7,
                _ => return Err(DecodeError {}),
            })?,
            Some(match move_index & 0b11 {
                0 => Role::Queen,
                1 => Role::Rook,
                2 => Role::Bishop,
                3 => Role::Knight,
                _ => unreachable!(),
            }),
        )
    } else {
        (
            new_square(match move_index {
                0 => from - 8,
                1 => from - 7,
                2 => from - 9,
                3 => from - 16,
                _ => return Err(DecodeError {}),
            })?,
            None,
        )
    })
}

fn destination_from_move_index(
    from: Square,
    role: Role,
    white: bool,
    move_index: u64,
) -> DecodeResult<(Square, Option<Role>)> {
    match role {
        Role::Knight => Ok((knight_destination_from_move_index(from, move_index)?, None)),
        Role::Rook => Ok((rook_destination_from_move_index(from, move_index), None)),
        Role::Bishop => Ok((bishop_destination_from_move_index(from, move_index)?, None)),
        Role::Queen => Ok((queen_destination_from_move_index(from, move_index)?, None)),
        Role::King => Ok((
            king_destination_from_move_index(from, white, move_index)?,
            None,
        )),
        Role::Pawn => pawn_destination_from_move_index(from, white, move_index),
    }
}

fn index_nth_set_bit(v: u64, n: u64) -> DecodeResult<u32> {
    let mut v = v;
    let mut count = 0;
    let mut i: u32 = 0;
    while v != 0 {
        if v & 1 != 0 {
            if count == n {
                return Ok(i);
            }
            count += 1;
        }
        v >>= 1;
        i += 1;
    }
    Err(DecodeError {})
}

pub struct CompactIndexEncoder {
    result: huff::EncodedGame,
    chess: shakmaty::Chess,
}

impl CompactIndexEncoder {
    pub fn new() -> Self {
        Self {
            result: huff::EncodedGame {
                inner: vec![0; 8],
                bit_index: 0,
            },
            chess: shakmaty::Chess::new(),
        }
    }
}

impl Default for CompactIndexEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Encode for CompactIndexEncoder {
    fn encode_move(&mut self, m: Move) -> Result<(), EncodeError> {
        let from = m.from().ok_or(EncodeError {
            inner: Box::new("missing from square in CompactIndexEncoder::encode_move"),
        })?;
        let turn = self.chess.turn();
        let white = turn == Color::White;
        let role = self
            .chess
            .board()
            .piece_at(from)
            .ok_or(EncodeError {
                inner: Box::new("missing piece at from square in CompactIndexEncoder::encode_move"),
            })?
            .role;

        let bb = if white {
            self.chess.board().white()
        } else {
            self.chess.board().black()
        };
        let piece_count = bb.count();
        let needed_bits = NEEDED_BITS[piece_count];

        // resize buffer if it may be too small
        if self.result.inner.len() * 64 < self.result.bit_index + 9 {
            self.result.inner.resize(self.result.inner.len() + 4, 0);
        }

        if needed_bits > 0 {
            let mask = u64::MAX >> (63 - from as u8);
            let index = (bb & mask).count() as u64 - 1;
            self.result
                .inner
                .set_bits(self.result.bit_index, index, needed_bits);
            self.result.bit_index += needed_bits as usize;
        }

        let (move_index, move_bits) = move_index_and_bits(from, m.to(), role, white, m.promotion());
        unsafe {
            self.result
                .inner
                .set_bits_unchecked(self.result.bit_index, move_index, move_bits);
        }
        self.result.bit_index += move_bits as usize;

        self.chess.play_unchecked(m);

        Ok(())
    }

    fn finish(self) -> EncodedGame<'static> {
        EncodedGame {
            content: EncodedGameContent::Bits(self.result),
            compression_level: crate::CompressionLevel::Medium,
        }
    }
}

pub struct CompactIndexDecoder<'a> {
    chess: shakmaty::Chess,
    encoded: &'a huff::EncodedGame,
    index: usize,
}

impl<'a> CompactIndexDecoder<'a> {
    pub(crate) fn new(encoded: &'a crate::EncodedGameContent<'a>) -> Self {
        if let crate::EncodedGameContent::Bits(enc) = encoded {
            Self {
                chess: shakmaty::Chess::new(),
                encoded: enc,
                index: 0,
            }
        } else {
            panic!("CompactIndexDecoder only accepts EncodedGameRef::Bits");
        }
    }
}

fn get_bits_checked(buffer: &[u64], begin: usize, len: u8) -> DecodeResult<u64> {
    buffer.try_get_bits(begin, len).ok_or(DecodeError {})
}

impl Decode for CompactIndexDecoder<'_> {
    fn next_move(&mut self) -> Option<DecodeResult<Move>> {
        if self.index == self.encoded.bit_index {
            return None;
        }

        let turn = self.chess.turn();
        let white = turn == Color::White;

        let bb = if white {
            self.chess.board().white()
        } else {
            self.chess.board().black()
        };
        let piece_count = bb.count();
        let square_bits = NEEDED_BITS[piece_count];

        let from = if square_bits > 0 {
            let square_index = match get_bits_checked(&self.encoded.inner, self.index, square_bits)
            {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            self.index += square_bits as usize;

            let bb = bb.0;

            match index_nth_set_bit(bb, square_index) {
                Ok(i) => unsafe { Square::new_unchecked(i) },
                Err(e) => return Some(Err(e)),
            }
        } else if let Some(sq) = bb.single_square() {
            sq
        } else {
            return Some(Err(DecodeError {}));
        };

        let role_at_from = self
            .chess
            .board()
            .role_at(from)
            .expect("role_at cannot be None due to how `from` is constructed");

        let move_bit_len = match role_at_from {
            Role::Knight | Role::King => 3,
            Role::Rook | Role::Bishop => 4,
            Role::Queen => 5,
            Role::Pawn => {
                if (white && from.rank() == Rank::Seventh)
                    || (!white && from.rank() == Rank::Second)
                {
                    4
                } else {
                    2
                }
            }
        };
        let move_bits = match get_bits_checked(&self.encoded.inner, self.index, move_bit_len) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        self.index += move_bit_len as usize;
        match destination_from_move_index(from, role_at_from, white, move_bits) {
            Ok((to, promotion)) => {
                let uci = UciMove::Normal {
                    from,
                    to,
                    promotion,
                };

                let r = uci.to_move(&self.chess).map_err(|_| DecodeError {});
                Some(r.map(|m| {
                    self.chess.play_unchecked(m); // uci.to_move already checks legality
                    m
                }))
            }
            Err(e) => Some(Err(e)),
        }
    }

    fn next_move_and_position(&mut self) -> Option<DecodeResult<(Move, &Chess)>> {
        let maybe_next = self.next_move();
        maybe_next.map(|next| next.map(|m| (m, &self.chess)))
    }

    fn next_position(&mut self) -> Option<DecodeResult<&Chess>> {
        let maybe_next = self.next_move();
        maybe_next.map(|next| next.map(|_| &self.chess))
    }
}

#[cfg(test)]
mod tests {
    use crate::{CompressionLevel, Decoder, EncodedGame};

    #[test]
    fn decode_test() {
        let bytes = b"<]\x93.\x0DT?\xE2\xEC\xDE\xEFaFR\x973\xDB\x03v";
        let encoded_game = EncodedGame::from_bytes(bytes).unwrap();
        assert_eq!(encoded_game.compression_level, CompressionLevel::Medium);

        let decoder = Decoder::new(&encoded_game);

        if let Decoder::CompactIndex(..) = &decoder {
            // expected
        } else {
            panic!("Decoder is not CompactIndex variant");
        }

        let uci = decoder.into_uci_string().unwrap();
        let expected_uci = "e2e4 e7e5 f1c4 b8c6 g1f3 b7b6 e1g1 g8f6 c2c3 f8c5 c4f7 e8f7 f3g5 f7g8 d1b3 f6d5 b3d5 g8f8 d5f7";
        assert_eq!(uci, expected_uci);
    }
}
