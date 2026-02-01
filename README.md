# Aix - `aixchess` DuckDB extension

Aix enables efficient storage and querying of large chess game collections. Read more on [my blog post](https://thomasd.be/2026/02/01/aix-storing-querying-chess-games.html) and the [documentation](docs/README.md).

Get started by:

* [Installing the `aixchess` extension](https://github.com/thomas-daniels/aix/releases/tag/v0.1.0) for DuckDB.
* Download one of the [Aix-compatible Lichess database files](https://huggingface.co/datasets/thomasd1/aix-lichess-database).

With the `aixchess` extension loaded, you can execute SQL queries over a chess game collection. For example, this query generates a heatmap of king move destinations:

```sql
with king_destinations as (
    select
        move_details(movedata)
            .list_filter(lambda m: m.role = 'k')
            .apply(lambda m: m.to)
        as destinations
    from 'aix_lichess_2025-12_low.parquet'
),
unnested as (
    select unnest(destinations) as destination from king_destinations
),
aggregated as (
    select destination, count() from unnested group by 1 order by 2 desc
)

from aggregated;
```

Which results in:

```
┌─────────────┬──────────────┐
│ destination │ count_star() │
│   varchar   │    int64     │
├─────────────┼──────────────┤
│ g1          │     74020594 │
│ g8          │     71579360 │
│ g7          │     23388424 │
...
```

The conversion from Lichess PGNs to Aix-compatible files is done using [`pgn-to-aix`](pgn-to-aix/README.md). At the moment, this tool is specifically tailored towards Lichess PGNs.

## Building the extension yourself

Make sure that CMake, [Ninja](https://ninja-build.org/), [ccache](https://ccache.dev/), and Cargo are installed. Build the extension using:

```
GEN=ninja make
```

A DuckDB binary with the extension loaded is then available in `./build/release/duckdb`. Running unit tests is possible with `make test`.
