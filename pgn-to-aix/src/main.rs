use std::fs::File;

use aix_chess_compression::CompressionLevel;
use clap::Parser;
use duckdb::Connection;
use pgn_reader;

mod args;
mod pgn;

fn main() {
    let args = args::Args::parse();

    let input_path = std::path::Path::new(&args.input);

    if !input_path.exists() {
        eprintln!("Input file '{}' does not exist.", input_path.display());
        std::process::exit(1);
    }

    let output_path = std::path::Path::new(&args.output);
    let output_duckdb = output_path.with_added_extension("duckdb");
    let output_parquet = output_path.with_added_extension("parquet");

    if output_duckdb.exists() {
        eprintln!(
            "Output DuckDB file '{}' already exists.",
            output_duckdb.display()
        );
        std::process::exit(1);
    }

    let db = Connection::open(output_duckdb).unwrap();

    if let Some(mem_limit) = args.duckdb_memory_limit_gb {
        db.execute("SET memory_limit = ?", [format!("{}G", mem_limit)])
            .unwrap();
    }

    db.execute_batch(include_str!("sql/init-database.sql"))
        .unwrap();
    let app = db.appender("games").unwrap();
    let mut proc = pgn::PgnProcessor::new(
        app,
        match args.compression {
            args::CompressionLevel::Low => CompressionLevel::Low,
            args::CompressionLevel::Medium => CompressionLevel::Medium,
            args::CompressionLevel::High => CompressionLevel::High,
        },
    );

    let file = File::open(input_path).unwrap();
    let uncompressed: Box<dyn std::io::Read> =
        if input_path.extension().and_then(|s| s.to_str()) == Some("zst") {
            Box::new(zstd::Decoder::new(file).unwrap())
        } else {
            Box::new(file)
        };

    let mut reader = pgn_reader::Reader::new(uncompressed);
    reader
        .read_games(&mut proc)
        .map(|e| e.unwrap())
        .for_each(drop);

    proc.flush();
    db.execute("checkpoint", []).unwrap();

    drop(proc);

    if !args.skip_parquet_export {
        println!("Exporting to Parquet file...");
        let parquet_compression_str = "COMPRESSION ".to_owned()
            + &(if args.parquet_compression == args::ParquetCompression::Zstd {
                format!(
                    "{}, COMPRESSION_LEVEL {}",
                    args.parquet_compression.as_str(),
                    args.parquet_compression_level
                )
            } else {
                args.parquet_compression.as_str().to_owned()
            });

        let to_parquet_command = format!(
            "COPY (FROM games) TO ? (FORMAT PARQUET, PARQUET_VERSION v2, {})",
            parquet_compression_str
        );

        db.execute(&to_parquet_command, [output_parquet.to_str().unwrap()])
            .unwrap();
    } else {
        println!("Skipping Parquet export.");
    }
}
