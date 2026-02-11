cmake_minimum_required(VERSION 3.16)
project(cc_rust_demo LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(RUST_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rustlib)

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Linux")
  message(FATAL_ERROR "This minimal demo targets Linux only.")
endif()

if(NOT CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64)$")
  message(FATAL_ERROR "This minimal demo targets x86_64 Linux only.")
endif()

set(RUST_DYLIB ${RUST_DIR}/target/release/librust_bridge.so)

# 先由 Cargo 构建 Rust 动态库（cdylib），再由 C++ 可执行程序链接它。
add_custom_command(
  OUTPUT ${RUST_DYLIB}
  COMMAND cargo build --release
  WORKING_DIRECTORY ${RUST_DIR}
  DEPENDS
    ${RUST_DIR}/Cargo.toml
    ${RUST_DIR}/src/lib.rs
  COMMENT "Building Rust cdylib with Cargo"
  VERBATIM
)

add_custom_target(rust_bridge ALL DEPENDS ${RUST_DYLIB})

add_executable(cpp_calls_rust main.cpp)
add_dependencies(cpp_calls_rust rust_bridge)
target_link_libraries(cpp_calls_rust PRIVATE ${RUST_DYLIB})
# Rust 动态库需要反向解析可执行程序里的 `cpp_mul` 符号。
target_link_options(cpp_calls_rust PRIVATE -Wl,--export-dynamic)

# 让可执行程序在构建目录运行时就能找到 Rust 动态库。
set_target_properties(cpp_calls_rust PROPERTIES
  BUILD_RPATH ${RUST_DIR}/target/release
)
