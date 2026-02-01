# Columns in Aix-compatible Lichess database

In general, the Aix extension does not prescribe how your database should be structured, and the provided [functions](functions.md) can be applied to any relevant column.

The [Aix-compatible Lichess database, available on Hugging Face](https://huggingface.co/datasets/thomasd1/aix-lichess-database), consists of Parquet files with the columns below. Some columns (e.g. `tournament`, `evals`) are not applicable for every game and can be `NULL`.

| Name              | Type              | Description              |
| ----------------- | ----------------- | ------------------------ |
| `lichess_id`      | `VARCHAR`         | ID of the game on Lichess, `https://lichess.org/<lichess_id>` |
| `tournament`      | `VARCHAR`         | ID of the Lichess tournament this game was played in (`https://lichess.org/tournament/<tournament>`). |
| `movedata`        | `BLOB`            | Binary representation of the moves in the game. |
| `clocks_white`    | `USMALLINT[]`     | List of white's clock times, in seconds. |
| `clocks_black`    | `USMALLINT[]`     | List of black's clock times, in seconds. |
| `evals`           | `SMALLINT[]`      | List of engine evaluations after each move (see [eval conversion functions](functions.md#eval_to_centipawnseval_to_mate)). |
| `ply_count`       | `USMALLINT`       | Number of plies (half-moves) in the game. |
| `white`           | `VARCHAR`         | Username of the white player. |
| `black`           | `VARCHAR`         | Username of the black player. |
| `white_rating`    | `SMALLINT`        | Rating of the white player. |
| `black_rating`    | `SMALLINT`        | Rating of the black player. |
| `time_initial`    | `USMALLINT`       | Initial clock time in seconds. (If the time control is Correspondence, both `time_initial` and `time_increcent` are `NULL`). |
| `time_increment`  | `UTINYINT`        | Clock increment in seconds. |
| `result`          | `VARCHAR`         | Game result (`1-0`, `0-1`, `1/2-1/2` or `*`). |
| `termination`     | `VARCHAR`         | How the game ended (e.g. `Normal`, `Time forfeit`). |
| `white_rating_diff` | `SMALLINT`      | Change in white's rating after the game. |
| `black_rating_diff` | `SMALLINT`      | Change in black's rating after the game. |
| `eco`             | `VARCHAR`         | [ECO code](https://en.wikipedia.org/wiki/List_of_ECO_codes) of the game's opening. |
| `opening`         | `VARCHAR`         | Name of the game's opening and variation. |
| `white_title`     | `VARCHAR`         | Master title of the white player. |
| `black_title`     | `VARCHAR`         | Master title of the black player. |
| `utc_timestamp`   | `TIMESTAMP`       | UTC timestamp when the game started. |