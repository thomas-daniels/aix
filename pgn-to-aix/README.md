# pgn-to-aix

Command-line tool to convert a PGN file into a Parquet file that can be used with Aix. Currently specifically tailored for [Lichess database PGNs](https://database.lichess.org/). Accepts uncompressed PGN files and zstd-compressed PGN files (with .zst extension).

```
Usage: pgn-to-aix [OPTIONS] --input <INPUT> --output <OUTPUT> --compression <COMPRESSION>

Options:
  -i, --input <INPUT>
          Path to the input file
  -o, --output <OUTPUT>
          Path to the output file(s) (without extension)
  -c, --compression <COMPRESSION>
          Compression level for movedata [possible values: low, medium, high]
      --skip-parquet-export
          Skip exporting to Parquet file
      --parquet-compression <PARQUET_COMPRESSION>
          Compression algorithm for output Parquet file [default: zstd] [possible values: uncompressed, snappy, gzip, zstd, brotli, lz4-raw]
      --parquet-compression-level <PARQUET_COMPRESSION_LEVEL>
          Compression level for output Parquet file (only relevant for zstd) [default: 19]
      --duckdb-memory-limit-gb <DUCKDB_MEMORY_LIMIT_GB>
          Optional DuckDB memory limit in GB
  -h, --help
          Print help
  -V, --version
          Print version
```

Example:

```
pgn-to-aix -i lichess_db_standard_rated_2013-01.pgn.zst -o aix_lichess_2013-01_low.parquet -c low --duckdb-memory-limit-gb 8
```
