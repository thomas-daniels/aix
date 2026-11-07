# This file is included by DuckDB's build system. It specifies which extension to load

# Extension from this repo
duckdb_extension_load(aixchess
    SOURCE_DIR ${CMAKE_CURRENT_LIST_DIR}
    LOAD_TESTS
    LINKED_LIBS "${CMAKE_SOURCE_DIR}/../aixrs/target/wasm32-unknown-emscripten/release/libaixrs.a"
)

# Any extra extensions that should be built
# e.g.: duckdb_extension_load(json)