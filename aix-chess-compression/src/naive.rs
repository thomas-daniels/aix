use std::borrow::Cow;

use crate::{
    CompressionLevel, Decode, DecodeError, DecodeResult, Encode, EncodeError, EncodedGame,
    EncodedGameContent,
};
use shakmaty::{Chess, Move, Position, Square, uci::UciMove};

pub struct NaiveEncoder {
    result: Vec<u8>,
}

impl NaiveEncoder {
    pub fn new() -> Self {
        Self {
            result: Vec::with_capacity(40),
        }
    }
}

impl Default for NaiveEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for NaiveEncoder {
    fn encode_move(&mut self, m: Move) -> Result<(), EncodeError> {
        let from: u8 = m
            .from()
            .ok_or(EncodeError {
                inner: Box::new("missing from square in NaiveEncoder::encode_move"),
            })?
            .into();
        let to: u8 = m.to().into();

        let mut b1 = from;
        let mut b2 = to;

        if m.is_capture() {
            b1 |= 0b1000_0000;
        }

        if let Some(promotion) = m.promotion() {
            b1 |= 0b0100_0000;
            b2 |= match promotion {
                shakmaty::Role::Queen => 0b0000_0000,
                shakmaty::Role::Rook => 0b0100_0000,
                shakmaty::Role::Bishop => 0b1000_0000,
                shakmaty::Role::Knight => 0b1100_0000,
                _ => panic!("Invalid promotion piece"),
            };
        }

        self.result.push(b1);
        self.result.push(b2);

        Ok(())
    }

    fn finish(self) -> EncodedGame<'static> {
        EncodedGame {
            content: EncodedGameContent::Bytes(Cow::Owned(self.result)),
            compression_level: CompressionLevel::Low,
        }
    }
}

pub struct NaiveDecoder<'a> {
    encoded: &'a [u8],
    index: usize,
    chess: Chess,
}

impl<'a> NaiveDecoder<'a> {
    pub(crate) fn new(encoded: &'a EncodedGameContent<'a>) -> Self {
        if let EncodedGameContent::Bytes(enc) = encoded {
            Self {
                encoded: enc,
                index: 0,
                chess: Chess::new(),
            }
        } else {
            panic!("NaiveDecoder only accepts EncodedGameRef::Bytes");
        }
    }
}

impl Decode for NaiveDecoder<'_> {
    fn next_move(&mut self) -> Option<DecodeResult<Move>> {
        if self.index == self.encoded.len() {
            return None;
        }

        if self.index + 1 == self.encoded.len() {
            return Some(Err(DecodeError {}));
        }

        let b1 = self.encoded[self.index];
        let b2 = self.encoded[self.index + 1];

        let from = unsafe { Square::new_unchecked(u32::from(b1 & 0b0011_1111)) };
        let to = unsafe { Square::new_unchecked(u32::from(b2 & 0b0011_1111)) };

        let promotion = if b1 & 0b0100_0000 != 0 {
            match b2 & 0b1100_0000 {
                0b0000_0000 => Some(shakmaty::Role::Queen),
                0b0100_0000 => Some(shakmaty::Role::Rook),
                0b1000_0000 => Some(shakmaty::Role::Bishop),
                0b1100_0000 => Some(shakmaty::Role::Knight),
                _ => unreachable!(),
            }
        } else {
            None
        };

        let uci = UciMove::Normal {
            from,
            to,
            promotion,
        };
        let r = uci.to_move(&self.chess).map_err(|_| DecodeError {});
        Some(r.map(|m| {
            self.chess.play_unchecked(m); // uci.to_move already checks legality
            self.index += 2;
            m
        }))
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
        let bytes = b"\x0C\x1C4$\x05\x1A9*\x06\x151)\x04\x07>-\x0A\x12=\x22\x9A5\xBC5\x15&5>\x03\x11-#\x91#>=#5\x00";
        let encoded_game = EncodedGame::from_bytes(bytes).unwrap();
        assert_eq!(encoded_game.compression_level, CompressionLevel::Low);

        let decoder = Decoder::new(&encoded_game);

        if let Decoder::Naive(..) = &decoder {
            // expected
        } else {
            panic!("Decoder is not Naive variant");
        }

        let uci = decoder.into_uci_string().unwrap();
        let expected_uci = "e2e4 e7e5 f1c4 b8c6 g1f3 b7b6 e1g1 g8f6 c2c3 f8c5 c4f7 e8f7 f3g5 f7g8 d1b3 f6d5 b3d5 g8f8 d5f7";
        assert_eq!(uci, expected_uci);
    }
}
