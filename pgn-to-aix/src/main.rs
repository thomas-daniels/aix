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

    if args.lichess && args.headers.is_some() {
        eprintln!("--headers and --lichess are mutually exclusive. Please choose one of them.");
        std::process::exit(1);
    }

    if !args.lichess && args.headers.is_none() {
        eprintln!(
            "When not using --lichess, you must provide a list of headers to include using --headers."
        );
        std::process::exit(1);
    }

    if !args.lichess && args.headers.as_ref().unwrap().is_empty() {
        eprintln!("The --headers list cannot be empty.");
        std::process::exit(1);
    }

    if let Some(headers) = &args.headers {
        if headers.iter().any(|h| h.trim().is_empty()) {
            eprintln!("The --headers list cannot contain empty header names.");
            std::process::exit(1);
        }
    }

    let db = Connection::open(output_duckdb).unwrap();

    if let Some(mem_limit) = args.duckdb_memory_limit_gb {
        db.execute("SET memory_limit = ?", [format!("{}G", mem_limit)])
            .unwrap();
    }

    let proc_headers_list = if args.lichess {
        db.execute_batch(include_str!("sql/init-lichess-database.sql"))
            .unwrap();

        None
    } else {
        let headers_sql = &args
            .headers
            .as_ref()
            .unwrap()
            .iter()
            .map(|h| format!("{} VARCHAR", h))
            .collect::<Vec<_>>()
            .join(",\n ");

        let init_sql =
            include_str!("sql/init-other-database.sql").replace("$HEADERS", &headers_sql);
        db.execute_batch(&init_sql).unwrap();

        Some(
            args.headers
                .as_ref()
                .unwrap()
                .iter()
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>(),
        )
    };

    let app = db.appender("games").unwrap();
    let mut proc = pgn::PgnProcessor::new(
        app,
        match args.compression {
            args::CompressionLevel::Low => CompressionLevel::Low,
            args::CompressionLevel::Medium => CompressionLevel::Medium,
            args::CompressionLevel::High => CompressionLevel::High,
        },
        proc_headers_list,
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
