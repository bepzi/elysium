cmake_minimum_required(VERSION 3.15)

project(ELYSIUM
  VERSION 0.1.0
  DESCRIPTION "A polyphonic synthesizer with Rust guts."
  LANGUAGES CXX)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})
include(defines)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# ==================================================

add_subdirectory(third_party/JUCE)
# add_subdirectory(elysium_ffi) # TODO: Does not contain a CMakeLists.txt yet. Should it?
add_subdirectory(elysium_cpp)
add_subdirectory(elysium_rust)

# ==================================================

get_target_property(cargo-dir elysium_rust CARGO_DIR)
get_target_property(cargo-flags elysium_rust CARGO_FLAGS)
set(cargo-toml ${CMAKE_SOURCE_DIR}/elysium_rust/Cargo.toml)

set(header-filter-regex "${CMAKE_SOURCE_DIR}/src/.+\.h(pp)?")
add_custom_target(lint
  COMMAND clang-tidy --header-filter=${header-filter-regex} ${Elysium_sources}
  COMMAND cargo clippy ${cargo-flags} --target-dir ${cargo-dir} --manifest-path ${cargo-toml}
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  COMMENT "Running 'clang-tidy' and 'cargo clippy' on C++ and Rust sources"
  VERBATIM)

add_dependencies(lint elysium_rust)

file(GLOB_RECURSE cxx-sources LIST_DIRECTORIES false
  "${CMAKE_SOURCE_DIR}/elysium_cpp/*.c??" "${CMAKE_SOURCE_DIR}/elysium_ffi/*.c??")
file(GLOB_RECURSE cxx-headers LIST_DIRECTORIES false
  "${CMAKE_SOURCE_DIR}/elysium_cpp/*.h??" "${CMAKE_SOURCE_DIR}/elysium_ffi/*.h??")

add_custom_target(format
  COMMAND clang-format -i -style=file ${cxx-sources} ${cxx-headers}
  COMMAND cargo fmt --manifest-path ${cargo-toml}
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  COMMENT "Running 'clang-format' and 'cargo fmt' on C++ and Rust sources"
  VERBATIM)
