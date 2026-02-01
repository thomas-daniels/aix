# Aix functions

The Aix extension provides several scalar functions in DuckDB that can be applied to encoded games, move times, or engine evaluations.

## board_at_position

`board_at_position(movedata BLOB, position INTEGER) -> STRUCT(a1 VARCHAR, b1 VARCHAR, c1 VARCHAR, ..., h8 VARCHAR)`

Returns the board at a given position as a struct. The starting position is 0. Negative integers are accepted, the final position is -1.


## clocks_to_move_times

`clocks_to_move_times(clocks USMALLINT[], increment UTINYINT) -> USMALLINT[]`

Transform clocks (of one player) into move times, given the clock times and increment.

Example: `select clocks_to_move_times([ 180, 175, 175, 150 ]::usmallint[], 1);` -> `[6, 1, 26]`


## eval_to_centipawns/eval_to_mate

Evaluations are represented as `SMALLINT`s (signed 16-bit integers, -32,768 to 32,767).  
The max value (32,767) represents mate in 1 for white, max - 511 (32,256) represents mate in 512 for white.  
The min value (-32,768) represents mate in 1 for black, min + 511 (-32,257) represents mate in 512 for black. 
Any other value represents an evaluation in centipawns.

`eval_to_centipawns(eval SMALLINT) -> SMALLINT` returns `eval` if it represents an evaluation in centipawns, `NULL` if it represents a mate.

`eval_to_mate(eval SMALLINT) -> SMALLINT` returns number of moves until mate (positive for white, negative for black) if `eval` represents a mate, `NULL` if it represents an evaluation in centipawns.

Use `list_eval_to_centipawns` or `list_eval_to_mate` to apply these functions to all items in a list.


## fen_at_position

`fen_at_position(movedata BLOB, position INTEGER) -> VARCHAR`

Returns the FEN at a given position. The starting position is 0. Negative integers are accepted, the final position is -1.


## matches_subfen

`matches_subfen(movedata BLOB, subfen VARCHAR) -> BOOLEAN`

Returns true if any position in the game matches a given sub-FEN.
A sub-FEN consists of only the piece placement part of a FEN (e.g. `8/8/p7/8/8/1B3N2/8/8`)
and matches if a position contains at least those pieces.


## move_details

`move_details(movedata BLOB) -> STRUCT(ply USMALLINT, role VARCHAR, from VARCHAR, to VARCHAR, promotion VARCHAR, capture VARCHAR, is_castle BOOLEAN, is_check BOOLEAN, is_checkmate BOOLEAN, is_en_passant BOOLEAN)[]`

Returns a list of details of all moves in the game. Note that lists in DuckDB are 1-indexed, so the first move is `move_details(...)[1]`.


## move_details_at

`move_details(movedata BLOB, index INTEGER) -> STRUCT(ply USMALLINT, role VARCHAR, from VARCHAR, to VARCHAR, promotion VARCHAR, capture VARCHAR, is_castle BOOLEAN, is_check BOOLEAN, is_checkmate BOOLEAN, is_en_passant BOOLEAN)`

Returns the details of a given move in the game. This function is 0-indexed, so the first move is `move_details_at(..., 0)`. This also means that `move_details_at(..., x) = move_details(...)[x + 1]`.

Negative indices are accepted, the last move is -1.


## moved_pieces

`moved_pieces(movedata BLOB) -> VARCHAR`

Returns the moved pieces in order as a string, e.g. `PpBnNpKnPbBkNkQnQkQ`.


## moved_pieces_list

`moved_pieces_list(movedata BLOB) -> VARCHAR[]`

Returns the moved pieces in order as a list, e.g. `[P, p, B, n, N, p, K, n, P, b, B, k, N, k, Q, n, Q, k, Q]`


## piece_counts_at_position

`piece_counts_at_position(movedata BLOB, position INTEGER) -> STRUCT(wQ UTINYINT, wR UTINYINT, wB UTINYINT, wN UTINYINT, wP UTINYINT, bQ UTINYINT, bR UTINYINT, bB UTINYINT, bN UTINYINT, bP UTINYINT)`

Returns the piece counts at a given position. The starting position is 0. Negative integers are accepted, the final position is -1.


## pieces_at_position

`pieces_at_position(movedata BLOB, position INTEGER) -> STRUCT(wK VARCHAR, wQ VARCHAR[], wR VARCHAR[], wB VARCHAR[], wN VARCHAR[], wP VARCHAR[], bK VARCHAR, bQ VARCHAR[], bR VARCHAR[], bB VARCHAR[], bN VARCHAR[], bP VARCHAR[])`

Returns the squares where the pieces are on at a given position. The starting position is 0. Negative integers are accepted, the final position is -1.


## recompress

`recompress(movedata BLOB, level UTINYINT) -> BLOB`

Recompress a game at a given compression level. Low is 0, medium is 1, high is 2.


## scoutfish_query

`scoutfish_query(movedata BLOB, query VARCHAR) -> BOOLEAN`

Returns true if a game matches a [Scoutfish](https://github.com/mcostalba/scoutfish) query.

The behavior of Aix does not entirely match that of Scoutfish, and that is by design:

* Aix does not support Scoutfish's `result` and `result-type` because this data is supposed to go in other columns (`movedata` does not have that data).
* Aix fixes [scoutfish#45](https://github.com/mcostalba/scoutfish/issues/45) and [scoutfish#56](https://github.com/mcostalba/scoutfish/issues/56).

There are likely more differences. If Aix's output does not match expectations, please open an issue.


## time_control_lichess

`time_control_lichess(initial_seconds USMALLINT, increment UTINYINT) -> VARCHAR`

Returns the time control name as on Lichess given the initial time and increment, both in seconds.
Possible return values: Ultrabullet, Bullet, Blitz, Rapid, Classical.


## to_pgn

`to_pgn(movedata BLOB) -> VARCHAR`

Represents the game as a PGN string, e.g. `1. e4 e5 2. Bc4 Nc6 3. Nf3 b6 4. O-O Nf6 5. c3 Bc5 6. Bxf7+ Kxf7 7. Ng5+ Kg8 8. Qb3+`.


## to_uci

`to_uci(movedata BLOB) -> VARCHAR`

Represents the game as a UCI string, e.g. `e2e4 e7e5 f1c4 b8c6 g1f3 b7b6 e1g1 g8f6 c2c3 f8c5 c4f7 e8f7 f3g5 f7g8 d1b3 f6d5 b3d5 g8f8 d5f7`.


## winning_chances_lichess

`winning_chances_lichess(SMALLINT) -> DOUBLE`

Converts a position evaluation into winning chances (as per Lichess's formula).
Equal position is `0`, white completely winning is `1`, black completely winning is `-1`.

Use `list_winning_chances_lichess(SMALLINT[])` to apply this function to all elements in a list.
