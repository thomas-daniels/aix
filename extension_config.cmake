# This file is included by DuckDB's build system. It specifies which extension to load

set (CMAKE_CXX_STANDARD 17)
add_compile_definitions(_HAS_STD_BYTE=0)

# Extension from this repo
duckdb_extension_load(aixchess
    SOURCE_DIR ${CMAKE_CURRENT_LIST_DIR}
    LOAD_TESTS
)

# Any extra extensions that should be built
# e.g.: duckdb_extension_load(json)