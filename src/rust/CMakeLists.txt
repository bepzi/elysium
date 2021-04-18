set(build-type $<IF:$<CONFIG:Debug>,debug,release>)

set(rust-lib-name "libelysium${CMAKE_STATIC_LIBRARY_SUFFIX}")
set(cargo-target-dir "${CMAKE_CURRENT_BINARY_DIR}/target")

set(rust-lib-path "${cargo-target-dir}/${build-type}/${rust-lib-name}")
set(cxxbridge-generated-cpp "${cargo-target-dir}/elysium_rust.cpp")
set(cxxbridge-generated-hpp "${cargo-target-dir}/elysium_rust.hpp")

file(GLOB_RECURSE rust-sources *.rs)
add_custom_command(
  OUTPUT ${cxxbridge-generated-cpp} ${cxxbridge-generated-hpp} ${rust-lib-path}
  MAIN_DEPENDENCY src/lib.rs
  DEPENDS ${rust-sources}
  COMMAND CARGO_TARGET_DIR=${cargo-target-dir} cargo build --${build-type}
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-target-dir}/cxxbridge/elysium/src/lib.rs.cc ${cxxbridge-generated-cpp}
  COMMAND ${CMAKE_COMMAND} -E copy ${cargo-target-dir}/cxxbridge/elysium/src/lib.rs.h ${cxxbridge-generated-hpp}
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
  VERBATIM)

add_library(elysium_rust STATIC
  ${cxxbridge-generated-cpp}
  wrappers/audio_buffer.cpp)

target_include_directories(elysium_rust
  # Other people want to find our generated C++ bindings to Rust
  # when they link to us, but we don't need them ourselves.
  INTERFACE ${cargo-target-dir}

  # We need to find our own headers to wrap JUCE C++ into bindable C++,
  # and the generated bindings will need to include them too.
  PUBLIC wrappers)

set_target_properties(elysium_rust PROPERTIES
  CXX_STANDARD 17
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

target_compile_definitions(elysium_rust PRIVATE
  JUCE_STANDALONE_APPLICATION=1)

target_link_libraries(elysium_rust
  PRIVATE
  ${CMAKE_DL_LIBS}
  ${rust-lib-path}

  PUBLIC
  juce::juce_audio_basics

  juce::juce_recommended_config_flags
  juce::juce_recommended_lto_flags)
