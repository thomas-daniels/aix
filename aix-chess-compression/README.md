[![crates.io](https://img.shields.io/crates/v/aix-chess-compression.svg)](https://crates.io/crates/aix-chess-compression)
[![docs.rs](https://docs.rs/aix-chess-compression/badge.svg)](https://docs.rs/aix-chess-compression)

# aix-chess-compression

Provides binary encoding and decoding for chess games, as used by [Aix](https://github.com/thomas-daniels/aix), but can also be used as standalone crate. Chess games can be encoded with one of three compression levels (Low, Medium, High).

Example of decoding a game:

```rust
use aix_chess_compression::{CompressionLevel, Decoder, EncodedGame};

// ...

let bytes = b"<]\x93.\x0DT?\xE2\xEC\xDE\xEFaFR\x973\xDB\x03v";

let encoded_game = EncodedGame::from_bytes(bytes).unwrap();
assert_eq!(encoded_game.compression_level, CompressionLevel::Medium);

let decoder = Decoder::new(&encoded_game);

// Decoder offers several functions to decode a game move by move, and utility functions like the one below

let uci = decoder.into_uci_string().unwrap();
let expected_uci = "e2e4 e7e5 f1c4 b8c6 g1f3 b7b6 e1g1 g8f6 c2c3 f8c5 c4f7 e8f7 f3g5 f7g8 d1b3 f6d5 b3d5 g8f8 d5f7";
assert_eq!(uci, expected_uci);
```

