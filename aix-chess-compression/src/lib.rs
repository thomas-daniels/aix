use chess_huffman::EncodedGame as BitsEncodedGame;
use shakmaty::{
    Chess, Move,
    san::{San, SanPlus, Suffix},
    uci::UciMove,
};
use std::{
    borrow::Cow,
    fmt::{self},
};

mod compactindex;
mod huffman;
mod naive;

use compactindex::{CompactIndexDecoder, CompactIndexEncoder};
use huffman::{HuffDecoder, HuffEncoder};
use naive::{NaiveDecoder, NaiveEncoder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompressionLevel {
    Low = 0,
    Medium = 1,
    High = 2,
}

const LEVELS: [CompressionLevel; 3] = [
    CompressionLevel::Low,
    CompressionLevel::Medium,
    CompressionLevel::High,
];

/// Encoder for chess games with different compression levels.
pub enum Encoder<'a> {
    Naive(NaiveEncoder),
    CompactIndex(CompactIndexEncoder),
    Huffman(HuffEncoder<'a>),
}

/// Encoded representation of a chess game.
#[derive(Clone, Debug)]
pub struct EncodedGame<'a> {
    content: EncodedGameContent<'a>,
    compression_level: CompressionLevel,
}

#[derive(Clone, Debug)]
pub(crate) enum EncodedGameContent<'a> {
    Bytes(Cow<'a, [u8]>),
    Bits(BitsEncodedGame),
}

/// Error type for constructing an `EncodedGame` from bytes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodedGameConstructionError {
    EmptyData = 0,
    InvalidCompressionLevel = 1,
    InvalidData = 2,
}

impl std::error::Error for EncodedGameConstructionError {}

impl fmt::Display for EncodedGameConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EncodedGameConstructionError::EmptyData =>
                    "Cannot construct EncodedGame from empty data",
                EncodedGameConstructionError::InvalidCompressionLevel =>
                    "Invalid compression level in EncodedGame data",
                EncodedGameConstructionError::InvalidData =>
                    "Invalid data for constructing EncodedGame",
            }
        )
    }
}

impl From<chess_huffman::EncodedGameConstructionError> for EncodedGameConstructionError {
    fn from(e: chess_huffman::EncodedGameConstructionError) -> Self {
        match e {
            chess_huffman::EncodedGameConstructionError::EmptyBytes => {
                EncodedGameConstructionError::EmptyData
            }
            chess_huffman::EncodedGameConstructionError::InvalidBytes => {
                EncodedGameConstructionError::InvalidData
            }
        }
    }
}

impl<'a> EncodedGame<'a> {
    /// Converts an encoded game into bytes. Use `from_bytes` to reconstruct.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = match self.content {
            EncodedGameContent::Bytes(bytes) => bytes.into_owned(),
            EncodedGameContent::Bits(bits) => bits.to_bytes(),
        };
        if self.compression_level == CompressionLevel::Low {
            bytes.push(0);
        } else {
            let len_minus_one = bytes.len() - 1;
            let last_byte = bytes[len_minus_one];
            bytes[len_minus_one] = last_byte | (self.compression_level as u8) << 6;
        }

        bytes
    }

    /// Constructs an encoded game from a byte slice produced by `into_bytes`.
    #[must_use]
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, EncodedGameConstructionError> {
        if bytes.is_empty() {
            return Err(EncodedGameConstructionError::EmptyData);
        }

        let len_minus_one = bytes.len() - 1;

        let level_bits = bytes[len_minus_one] >> 6;
        if level_bits > 2 {
            return Err(EncodedGameConstructionError::InvalidCompressionLevel);
        }

        let level = LEVELS[(bytes[len_minus_one] >> 6) as usize];

        let content = if level == CompressionLevel::Low {
            EncodedGameContent::Bytes(Cow::Borrowed(&bytes[..len_minus_one]))
        } else {
            EncodedGameContent::Bits(BitsEncodedGame::from_bytes(bytes)?)
        };

        Ok(EncodedGame {
            content,
            compression_level: level,
        })
    }

    /// Constructs an encoded game from an owned byte vector produced by `into_bytes`.
    #[must_use]
    pub fn from_owned_bytes(mut bytes: Vec<u8>) -> Result<Self, EncodedGameConstructionError> {
        let len_minus_one = bytes.len() - 1;

        let level = LEVELS[(bytes[len_minus_one] >> 6) as usize];

        let content = if level == CompressionLevel::Low {
            bytes.pop();
            EncodedGameContent::Bytes(Cow::Owned(bytes))
        } else {
            EncodedGameContent::Bits(BitsEncodedGame::from_bytes(&bytes)?)
        };

        Ok(EncodedGame {
            content,
            compression_level: level,
        })
    }

    /// Recompresses the encoded game into a different compression level.
    #[must_use]
    pub fn recompress(self, level: CompressionLevel) -> DecodeResult<Self> {
        let mut encoder = Encoder::new(level);
        let mut decoder = Decoder::new(&self);
        while let Some(m) = decoder.next_move() {
            encoder.encode_move(m?).expect("Encoding in recompress() failed, which should not happen because decoding succeeded");
        }
        Ok(encoder.finish())
    }
}

impl Encoder<'_> {
    /// Creates a new encoder for the specified compression level.
    #[must_use]
    pub fn new(compression_level: CompressionLevel) -> Self {
        match compression_level {
            CompressionLevel::Low => Encoder::Naive(NaiveEncoder::new()),
            CompressionLevel::Medium => Encoder::CompactIndex(CompactIndexEncoder::new()),
            CompressionLevel::High => Encoder::Huffman(HuffEncoder::new()),
        }
    }
}

impl Encode for Encoder<'_> {
    fn encode_move(&mut self, m: Move) -> Result<(), EncodeError> {
        match self {
            Encoder::Naive(enc) => enc.encode_move(m),
            Encoder::CompactIndex(enc) => enc.encode_move(m),
            Encoder::Huffman(enc) => enc.encode_move(m),
        }
    }

    fn finish(self) -> EncodedGame<'static> {
        match self {
            Encoder::Naive(enc) => enc.finish(),
            Encoder::CompactIndex(enc) => enc.finish(),
            Encoder::Huffman(enc) => enc.finish(),
        }
    }
}

pub trait Encode {
    /// Encodes a move into the game.
    fn encode_move(&mut self, m: Move) -> Result<(), EncodeError>;
    /// Finalizes the encoding and returns the encoded game.
    fn finish(self) -> EncodedGame<'static>;
}

pub enum Decoder<'a> {
    Naive(NaiveDecoder<'a>),
    CompactIndex(CompactIndexDecoder<'a>),
    Huffman(HuffDecoder<'a>),
}

impl<'a> Decoder<'a> {
    /// Creates a new decoder for an encoded game.
    #[must_use]
    pub fn new(encoded: &'a EncodedGame) -> Self {
        match encoded.compression_level {
            CompressionLevel::Low => Decoder::Naive(NaiveDecoder::new(&encoded.content)),
            CompressionLevel::Medium => {
                Decoder::CompactIndex(CompactIndexDecoder::new(&encoded.content))
            }
            CompressionLevel::High => Decoder::Huffman(HuffDecoder::new(&encoded.content)),
        }
    }

    /// Decodes all moves and represents the game as a UCI string.
    pub fn into_uci_string(self) -> DecodeResult<String>
    where
        Self: Sized,
    {
        let mut s = String::new();
        let mut first = true;
        for m in self.into_iter_moves() {
            if !first {
                s.push(' ');
            }
            first = false;
            s.push_str(&UciMove::from_standard(m?).to_string());
        }
        Ok(s)
    }

    /// Decodes all moves and represents the game as a PGN string.
    pub fn into_pgn_string(self) -> DecodeResult<String>
    where
        Self: Sized,
    {
        let mut s = String::new();
        let mut pos = Chess::new();
        let mut first = true;
        let mut i = 2;
        for r in self.into_iter_moves_and_positions() {
            let (m, next_pos) = r?;
            if !first {
                s.push(' ');
            }
            first = false;

            if i % 2 == 0 {
                let move_number = i / 2;
                s.push_str(&format!("{}. ", move_number));
            }
            i += 1;

            let san = San::from_move(&pos, m);
            let suffix = Suffix::from_position(&next_pos);
            let san_plus = SanPlus { san, suffix };
            san_plus.append_to_string(&mut s);

            pos = next_pos;
        }

        Ok(s)
    }

    /// Decodes all moves and positions into vectors.
    pub fn decode_all_moves_and_positions(self) -> DecodeResult<(Vec<Move>, Vec<Chess>)> {
        let mut moves = vec![];
        let mut positions = vec![];

        for d in self.into_iter_moves_and_positions() {
            let (m, pos) = d?;
            moves.push(m);
            positions.push(pos);
        }

        Ok((moves, positions))
    }
}

impl Decode for Decoder<'_> {
    fn next_move(&mut self) -> Option<DecodeResult<Move>> {
        match self {
            Decoder::Naive(decoder) => decoder.next_move(),
            Decoder::CompactIndex(decoder) => decoder.next_move(),
            Decoder::Huffman(decoder) => decoder.next_move(),
        }
    }

    fn next_position(&mut self) -> Option<DecodeResult<&Chess>> {
        match self {
            Decoder::Naive(decoder) => decoder.next_position(),
            Decoder::CompactIndex(decoder) => decoder.next_position(),
            Decoder::Huffman(decoder) => decoder.next_position(),
        }
    }

    fn next_move_and_position(&mut self) -> Option<DecodeResult<(Move, &Chess)>> {
        match self {
            Decoder::Naive(decoder) => decoder.next_move_and_position(),
            Decoder::CompactIndex(decoder) => decoder.next_move_and_position(),
            Decoder::Huffman(decoder) => decoder.next_move_and_position(),
        }
    }
}

pub trait Decode {
    /// Decodes the next move and returns it.
    fn next_move(&mut self) -> Option<DecodeResult<Move>>;
    /// Decodes the next move and returns the position after it.
    fn next_position(&mut self) -> Option<DecodeResult<&Chess>>;
    /// Decodes the next move and returns it along with the position after it.
    fn next_move_and_position(&mut self) -> Option<DecodeResult<(Move, &Chess)>>;

    /// Converts the decoder into an iterator over moves.
    fn into_iter_moves(self) -> impl Iterator<Item = DecodeResult<Move>>
    where
        Self: Sized,
    {
        struct MoveIter<T> {
            decoder: T,
            error: bool,
        }
        impl<T: Decode> Iterator for MoveIter<T> {
            type Item = DecodeResult<Move>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.error {
                    return Some(Err(DecodeError {}));
                }

                let item = self.decoder.next_move();
                if let Some(Err(_)) = item {
                    self.error = true;
                }
                item
            }
        }

        MoveIter {
            decoder: self,
            error: false,
        }
    }

    /// Converts the decoder into an iterator over positions.
    fn into_iter_positions(self) -> impl Iterator<Item = DecodeResult<Chess>>
    where
        Self: Sized,
    {
        struct PosIter<T> {
            decoder: T,
            error: bool,
        }
        impl<T: Decode> Iterator for PosIter<T> {
            type Item = DecodeResult<Chess>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.error {
                    return Some(Err(DecodeError {}));
                }

                let item = self.decoder.next_position().map(|res| res.cloned());
                if let Some(Err(_)) = item {
                    self.error = true;
                }
                item
            }
        }

        PosIter {
            decoder: self,
            error: false,
        }
    }

    /// Converts the decoder into an iterator over moves and positions.
    fn into_iter_moves_and_positions(self) -> impl Iterator<Item = DecodeResult<(Move, Chess)>>
    where
        Self: Sized,
    {
        struct MovePosIter<T> {
            decoder: T,
            error: bool,
        }
        impl<T: Decode> Iterator for MovePosIter<T> {
            type Item = DecodeResult<(Move, Chess)>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.error {
                    return Some(Err(DecodeError {}));
                }

                let item = self
                    .decoder
                    .next_move_and_position()
                    .map(|r| r.map(|(m, p)| (m, p.clone())));
                if let Some(Err(_)) = item {
                    self.error = true;
                }
                item
            }
        }

        MovePosIter {
            decoder: self,
            error: false,
        }
    }
}

/// Error type for decoding failures.
#[derive(Clone, Debug)]
pub struct DecodeError {}

impl std::error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot decode invalid game data")
    }
}

/// Error type for decoding failures.
#[derive(Debug)]
pub struct EncodeError {
    inner: Box<dyn std::fmt::Debug + Send + Sync>,
}

impl std::error::Error for EncodeError {}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to encode move: {:?}", self.inner)
    }
}

impl EncodeError {
    fn from_inner<E: std::fmt::Debug + Send + Sync + 'static>(err: E) -> Self {
        EncodeError {
            inner: Box::new(err),
        }
    }
}

/// Result type for decoding operations.
pub type DecodeResult<T> = Result<T, DecodeError>;

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use shakmaty::{Chess, Move, Position};

    use crate::Decode;

    use super::{CompressionLevel, Decoder, Encode, EncodedGame, Encoder};

    fn random_games_consistency(move_ids: Vec<u16>, level: CompressionLevel) -> bool {
        let mut pos = Chess::default();
        let mut moves: Vec<Move> = vec![];
        let mut positions: Vec<Chess> = vec![];

        for m in move_ids {
            let legal_moves = pos.legal_moves();
            if legal_moves.is_empty() {
                break;
            }

            let i = m as usize % legal_moves.len();
            let choice = legal_moves[i];
            pos.play_unchecked(choice);
            moves.push(choice);
            positions.push(pos.clone());
        }

        let mut encoder = Encoder::new(level);
        for &m in &moves {
            encoder.encode_move(m).unwrap();
        }

        let encoded = encoder.finish();
        if encoded.compression_level != level {
            panic!("encoded.compression_level != level");
        }

        let bytes = encoded.clone().into_bytes();

        let restored = EncodedGame::from_bytes(&bytes).unwrap();
        if restored.compression_level != encoded.compression_level {
            panic!("restored.compression_level != encoded.compression_level");
        }

        let decoder = Decoder::new(&restored);
        let restored_moves: Vec<Move> = decoder.into_iter_moves().map(|m| m.unwrap()).collect();

        let decoder2 = Decoder::new(&restored);
        let restored_positions: Vec<Chess> =
            decoder2.into_iter_positions().map(|p| p.unwrap()).collect();

        if moves != restored_moves {
            panic!("restored_moves != moves");
        }

        if positions != restored_positions {
            panic!("restored_positions != positions");
        }

        true
    }

    #[quickcheck]
    fn random_games_consistency_low(move_ids: Vec<u16>) -> bool {
        random_games_consistency(move_ids, CompressionLevel::Low)
    }

    #[quickcheck]
    fn random_games_consistency_medium(move_ids: Vec<u16>) -> bool {
        random_games_consistency(move_ids, CompressionLevel::Medium)
    }

    #[quickcheck]
    fn random_games_consistency_high(move_ids: Vec<u16>) -> bool {
        random_games_consistency(move_ids, CompressionLevel::High)
    }

    #[quickcheck]
    fn no_decode_panics(data: Vec<u8>) -> bool {
        match EncodedGame::from_bytes(&data) {
            Ok(encoded) => {
                let mut decoder = Decoder::new(&encoded);
                while let Some(m) = decoder.next_move() {
                    assert!(m.is_ok() || m.is_err());
                    if m.is_err() {
                        break;
                    }
                }
            }
            Err(_) => {}
        }
        true
    }
}
