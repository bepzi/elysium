set(build-type $<IF:$<CONFIG:Debug>,debug,release>)

set(rust-lib-name "libelysium_core${CMAKE_STATIC_LIBRARY_SUFFIX}")
set(cargo-target-dir "${CMAKE_CURRENT_BINARY_DIR}/target")

set(rust-lib-path "${cargo-target-dir}/${build-type}/${rust-lib-name}")
set(cxxbridge-generated-cpp "${cargo-target-dir}/elysium_rust.cpp")
set(cxxbridge-generated-hpp "${cargo-target-dir}/elysium_rust.hpp")

add_custom_command(
  OUTPUT ${cxxbridge-generated-cpp} ${cxxbridge-generated-hpp} ${compiled-rust-lib-path}
  MAIN_DEPENDENCY lib.rs
  COMMAND CARGO_TARGET_DIR=${cargo-target-dir} cargo build --${build-type}
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-target-dir}/cxxbridge/elysium-core/lib.rs.cc ${cxxbridge-generated-cpp}
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-target-dir}/cxxbridge/elysium-core/lib.rs.h ${cxxbridge-generated-hpp}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  VERBATIM)

add_library(elysium_rust STATIC ${cxxbridge-generated-cpp} wrapper.cpp)

target_include_directories(elysium_rust
  # We need to find our own headers to wrap JUCE C++ to Rustic C++.
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}
  # Other people want to find our generated C++ bindings to Rust.
  INTERFACE ${cargo-target-dir})

set_target_properties(elysium_rust PROPERTIES
  CXX_STANDARD 17
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

target_link_libraries(elysium_rust
  PRIVATE
  ${CMAKE_DL_LIBS}
  ${rust-lib-path}
  PUBLIC
  juce::juce_audio_basics)
