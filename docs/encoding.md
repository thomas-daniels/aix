# Binary encoding of chess games

Aix makes use of a binary encoding for the moves in a chess game (the [movedata column](columns.md) in the Aix-compatible Lichess database).

There are three possible compression levels for the binary encoding: Low, Medium, and High. A lower compression level takes up more disk space, but the decoding speed is higher. The [`recompress` function](functions.md#recompress) can transform encoded games between different compression levels.
