use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(super) struct Args {
    /// Path to the input file, PGN or zstd-compressed PGN (.zst)
    #[arg(short, long)]
    pub input: String,

    /// Path to the output file(s) (without extension)
    #[arg(short, long)]
    pub output: String,

    /// Compression level for movedata
    #[arg(short, long, value_enum)]
    pub compression: CompressionLevel,

    /// Skip exporting to Parquet file
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub skip_parquet_export: bool,

    /// Compression algorithm for output Parquet file
    #[arg(long, value_enum, default_value_t = ParquetCompression::Zstd)]
    pub parquet_compression: ParquetCompression,

    /// Compression level for output Parquet file (only relevant for zstd)
    #[arg(long, default_value_t = 19, value_parser=clap::value_parser!(u8).range(1..=22),)]
    pub parquet_compression_level: u8,

    /// Optional DuckDB memory limit in GB
    #[arg(long)]
    pub duckdb_memory_limit_gb: Option<u16>,

    /// Parse the input file as Lichess database file. This automatically handles the PGN headers correctly.
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub lichess: bool,

    /// Comma-separated list of PGN heaeders to include in the output database. Only relevant when not using --lichess.
    #[arg(long, value_delimiter = ',')]
    pub headers: Option<Vec<String>>,

    /// Set this flag to continue processing even if an invalid move is encountered in a game, rather than exiting with an error. The game with the invalid move will end right before the invalid move.
    #[arg(long, default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub continue_on_invalid_move: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub(super) enum CompressionLevel {
    Low,
    Medium,
    High,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub(super) enum ParquetCompression {
    Uncompressed,
    Snappy,
    Gzip,
    Zstd,
    Brotli,
    Lz4Raw,
}

impl ParquetCompression {
    pub fn as_str(&self) -> &str {
        match self {
            ParquetCompression::Uncompressed => "uncompressed",
            ParquetCompression::Snappy => "snappy",
            ParquetCompression::Gzip => "gzip",
            ParquetCompression::Zstd => "zstd",
            ParquetCompression::Brotli => "brotli",
            ParquetCompression::Lz4Raw => "lz4_raw",
        }
    }
}
