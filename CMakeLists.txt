cmake_minimum_required(VERSION 3.5)

# Set extension name here
set(TARGET_NAME aixchess)

set(EXTENSION_NAME ${TARGET_NAME}_extension)
set(LOADABLE_EXTENSION_NAME ${TARGET_NAME}_loadable_extension)

if(WIN32)
  set(RUST_LIB_FILENAME aixrs.lib)
else()
  set(RUST_LIB_FILENAME libaixrs.a)
endif()

if (DEFINED Rust_CARGO_TARGET)
  set(RUST_LIB_DEST ${CMAKE_SOURCE_DIR}/../aixrs/target/${Rust_CARGO_TARGET}/release/${RUST_LIB_FILENAME})
  set(RUST_BUILD_CMD cargo build --release --target ${Rust_CARGO_TARGET})
else()
  set(RUST_LIB_DEST ${CMAKE_SOURCE_DIR}/../aixrs/target/release/${RUST_LIB_FILENAME})
  set(RUST_BUILD_CMD cargo build --release)
endif()

add_custom_target(
  rust_lib
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/../aixrs
  COMMAND ${RUST_BUILD_CMD}
  BYPRODUCTS ${RUST_LIB_DEST}
)

project(${TARGET_NAME})
include_directories(src/include)

file(GLOB_RECURSE EXTENSION_SOURCES "src/*.cpp")

build_static_extension(${TARGET_NAME} ${EXTENSION_SOURCES})
build_loadable_extension(${TARGET_NAME} " " ${EXTENSION_SOURCES})

add_dependencies(${EXTENSION_NAME} rust_lib)
add_dependencies(${LOADABLE_EXTENSION_NAME} rust_lib)
target_link_libraries(${EXTENSION_NAME} ${RUST_LIB_DEST})
target_link_libraries(${LOADABLE_EXTENSION_NAME} ${RUST_LIB_DEST})

if (WIN32)
  target_link_libraries(${EXTENSION_NAME} kernel32.lib)
  target_link_libraries(${EXTENSION_NAME} ntdll.lib)
  target_link_libraries(${EXTENSION_NAME} userenv.lib)
  target_link_libraries(${EXTENSION_NAME} ws2_32.lib)
  target_link_libraries(${EXTENSION_NAME} dbghelp.lib)

  target_link_libraries(${LOADABLE_EXTENSION_NAME} kernel32.lib)
  target_link_libraries(${LOADABLE_EXTENSION_NAME} ntdll.lib)
  target_link_libraries(${LOADABLE_EXTENSION_NAME} userenv.lib)
  target_link_libraries(${LOADABLE_EXTENSION_NAME} ws2_32.lib)
  target_link_libraries(${LOADABLE_EXTENSION_NAME} dbghelp.lib)
endif()

install(
  TARGETS ${EXTENSION_NAME}
  EXPORT "${DUCKDB_EXPORT_SET}"
  LIBRARY DESTINATION "${INSTALL_LIB_DIR}"
  ARCHIVE DESTINATION "${INSTALL_LIB_DIR}")
