# pgn-to-aix

Command-line tool to convert a PGN file into a Parquet file that can be used with [Aix](https://github.com/thomas-daniels/aix). Accepts uncompressed PGN files and zstd-compressed PGN files (with .zst extension). `pgn-to-aix` will generate a DuckDB file (`.duckdb`) and, unless disabled, a Parquet file (`.parquet`).

If your input is a [Lichess database PGNs](https://database.lichess.org/) file, use the `--lichess` flag. Some months have [illegal castling moves](https://github.com/lichess-org/database/issues/23), you also need the `--continue-on-invalid-move` flag to process these.

If your input is another PGN file, use `--headers Header1,Header2,...` to specify which headers you want to include. E.g., `--headers White,Black,Round`. These headers will be included as VARCHAR columns in the output.

```
Usage: pgn-to-aix [OPTIONS] --input <INPUT> --output <OUTPUT> --compression <COMPRESSION>

Options:
  -i, --input <INPUT>
          Path to the input file, PGN or zstd-compressed PGN (.zst)
  -o, --output <OUTPUT>
          Path to the output file(s) (without extension)
  -c, --compression <COMPRESSION>
          Compression level for movedata [possible values: low, medium, high]
      --lichess
          Parse the input file as Lichess database file. This automatically handles the PGN headers correctly
      --headers <HEADERS>
          Comma-separated list of PGN heaeders to include in the output database. Only relevant when not using --lichess
      --skip-parquet-export
          Skip exporting to Parquet file
      --parquet-compression <PARQUET_COMPRESSION>
          Compression algorithm for output Parquet file [default: zstd] [possible values: uncompressed, snappy, gzip, zstd, brotli, lz4-raw]
      --parquet-compression-level <PARQUET_COMPRESSION_LEVEL>
          Compression level for output Parquet file (only relevant for zstd) [default: 19]
      --duckdb-memory-limit-gb <DUCKDB_MEMORY_LIMIT_GB>
          Optional DuckDB memory limit in GB
      --continue-on-invalid-move
          Set this flag to continue processing even if an invalid move is encountered in a game, rather than exiting with an error. The game with the invalid move will end right before the invalid move
  -h, --help
          Print help
  -V, --version
          Print version
```

Example:

```
pgn-to-aix -i lichess_db_standard_rated_2013-01.pgn.zst -o aix_lichess_2013-01_low.parquet -c low --lichess --duckdb-memory-limit-gb 8
```
