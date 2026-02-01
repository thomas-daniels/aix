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
