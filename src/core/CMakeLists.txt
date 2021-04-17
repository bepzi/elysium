set(compiled-rust-lib "libelysium_core${CMAKE_STATIC_LIBRARY_SUFFIX}")

set(compiled-rust-lib-path "${CMAKE_CURRENT_BINARY_DIR}/${compiled-rust-lib}")
set(cxxbridge-generated-cpp "${CMAKE_CURRENT_BINARY_DIR}/elysium_rust.cpp")
set(cxxbridge-generated-hpp "${CMAKE_CURRENT_BINARY_DIR}/elysium_rust.hpp")

set(cargo-output-dir "${CMAKE_CURRENT_SOURCE_DIR}/target")

add_custom_command(
  OUTPUT ${cxxbridge-generated-cpp} ${cxxbridge-generated-hpp} ${compiled-rust-lib-path}
  MAIN_DEPENDENCY lib.rs
  COMMAND cargo build --release # --verbose
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-output-dir}/release/${compiled-rust-lib} ${compiled-rust-lib-path}
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-output-dir}/cxxbridge/elysium-core/lib.rs.cc ${cxxbridge-generated-cpp}
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-output-dir}/cxxbridge/elysium-core/lib.rs.h ${cxxbridge-generated-hpp}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  VERBATIM)

add_library(elysium-rust-core STATIC ${cxxbridge-generated-cpp} wrapper.cpp)

target_include_directories(elysium-rust-core
  # We need to find our own headers to wrap JUCE C++ to Rustic C++.
  PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}
  # Other people want to find our generated C++ bindings to Rust.
  INTERFACE ${CMAKE_CURRENT_BINARY_DIR})

set_target_properties(elysium-rust-core PROPERTIES
  CXX_STANDARD 17
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

target_link_libraries(elysium-rust-core
  PRIVATE
  ${CMAKE_DL_LIBS}
  ${compiled-rust-lib-path}
  PUBLIC
  juce::juce_audio_basics)
