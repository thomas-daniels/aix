# This file is included by DuckDB's build system. It specifies which extension to load

# Extension from this repo
if (DEFINED Rust_CARGO_TARGET)
  set(AIXCHESS_WASM_RUST_LIB_DEST ${CMAKE_SOURCE_DIR}/../aixrs/target/${Rust_CARGO_TARGET}/release/libaixrs.a)
else()
  set(AIXCHESS_WASM_RUST_LIB_DEST ${CMAKE_SOURCE_DIR}/../aixrs/target/release/libaixrs.a)
endif()

duckdb_extension_load(aixchess
    SOURCE_DIR ${CMAKE_CURRENT_LIST_DIR}
    LOAD_TESTS
    LINKED_LIBS ${AIXCHESS_WASM_RUST_LIB_DEST}
)

# Any extra extensions that should be built
# e.g.: duckdb_extension_load(json)