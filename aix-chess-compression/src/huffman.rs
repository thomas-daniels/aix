use crate::{
    CompressionLevel, Decode, DecodeError, DecodeResult, Encode, EncodeError, EncodedGame,
    EncodedGameContent,
};
use chess_huffman as huff;
use shakmaty::{Chess, Move};

pub struct HuffEncoder<'a> {
    inner: huff::MoveByMoveEncoder<'a>,
}

impl HuffEncoder<'_> {
    pub fn new() -> Self {
        Self {
            inner: huff::MoveByMoveEncoder::new(),
        }
    }
}

impl Default for HuffEncoder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for HuffEncoder<'_> {
    fn encode_move(&mut self, m: Move) -> Result<(), EncodeError> {
        self.inner
            .add_move(m)
            .map_err(|e| EncodeError::from_inner(e))?;
        Ok(())
    }

    fn finish(self) -> EncodedGame<'static> {
        EncodedGame {
            content: EncodedGameContent::Bits(self.inner.result),
            compression_level: CompressionLevel::High,
        }
    }
}

pub struct HuffDecoder<'a> {
    inner: huff::MoveByMoveDecoder<'a>,
}

impl<'a> HuffDecoder<'a> {
    pub(crate) fn new(encoded: &'a EncodedGameContent<'a>) -> Self {
        if let EncodedGameContent::Bits(enc) = encoded {
            Self {
                inner: huff::MoveByMoveDecoder::new(enc),
            }
        } else {
            panic!("HuffDecoder only accepts EncodedGameRef::Bits");
        }
    }
}

impl Decode for HuffDecoder<'_> {
    fn next_move(&mut self) -> Option<DecodeResult<Move>> {
        self.inner
            .next_move()
            .map(|m| m.map_err(|_| DecodeError {}))
    }

    fn next_move_and_position(&mut self) -> Option<DecodeResult<(Move, &Chess)>> {
        self.inner
            .next_move_and_position()
            .map(|m| m.map_err(|_| DecodeError {}))
    }

    fn next_position(&mut self) -> Option<DecodeResult<&Chess>> {
        self.inner
            .next_position()
            .map(|m| m.map_err(|_| DecodeError {}))
    }
}

#[cfg(test)]
mod tests {
    use crate::{CompressionLevel, Decoder, EncodedGame};

    #[test]
    fn decode_test() {
        let bytes = b"\x9F-\x84\x1C\x1A\x9D:\xBD\xB3\xB8";
        let encoded_game = EncodedGame::from_bytes(bytes).unwrap();
        assert_eq!(encoded_game.compression_level, CompressionLevel::High);

        let decoder = Decoder::new(&encoded_game);

        if let Decoder::Huffman(..) = &decoder {
            // expected
        } else {
            panic!("Decoder is not Huffman variant");
        }

        let uci = decoder.into_uci_string().unwrap();
        let expected_uci = "e2e4 e7e5 f1c4 b8c6 g1f3 b7b6 e1g1 g8f6 c2c3 f8c5 c4f7 e8f7 f3g5 f7g8 d1b3 f6d5 b3d5 g8f8 d5f7";
        assert_eq!(uci, expected_uci);
    }
}
