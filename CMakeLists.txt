cmake_minimum_required(VERSION 3.15)

# Set a default build type if none was specified
set(default_build_type "RelWithDebInfo")

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "Setting build type to '${default_build_type}' as none was specified.")
  set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE
      STRING "Choose the type of build." FORCE)
  # Set the possible values of build type for cmake-gui
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
    "Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()

project(ELYSIUM
  VERSION 0.1.0
  DESCRIPTION "A polyphonic synthesizer with Rust guts."
  LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# ==================================================

add_subdirectory(third_party/JUCE)
add_subdirectory(src/rust)

# ==================================================

juce_add_plugin(Elysium
  IS_SYNTH TRUE
  NEEDS_MIDI_INPUT TRUE
  NEEDS_MIDI_OUTPUT FALSE
  IS_MIDI_EFFECT FALSE
  EDITOR_WANTS_KEYBOARD_FOCUS TRUE

  PRODUCT_NAME "Elysium"
  COMPANY_NAME "bepzi"
  PLUGIN_MANUFACTURER_CODE "B3pz"
  FORMATS LV2 Standalone

  LV2_URI "http://bepzi.com/elysium"
  LV2_WANTS_LATENCY FALSE)

set_target_properties(Elysium PROPERTIES
  CXX_STANDARD 20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

target_sources(Elysium PRIVATE
  src/ui/elysium.cpp)

target_include_directories(Elysium PUBLIC
  src/ui)

target_compile_definitions(Elysium PUBLIC
  JUCE_DISPLAY_SPLASH_SCREEN=0
  JUCE_VST3_CAN_REPLACE_VST2=0
  JUCE_WEB_BROWSER=0
  JUCE_USE_CURL=0
  JUCE_USE_XRANDR=0
  JUCE_WASAPI=1
  JUCE_DIRECTSOUND=1
  JUCE_ALSA=1
  JUCE_JACK=1
  JUCE_JACK_CLIENT_NAME="Elysium"
  JUCE_ALSA_MIDI_NAME="Elysium")

target_link_libraries(Elysium
  PRIVATE
  juce::juce_audio_basics
  juce::juce_audio_devices
  # juce::juce_audio_formats
  juce::juce_audio_plugin_client
  juce::juce_audio_processors
  juce::juce_audio_utils
  juce::juce_core
  # juce::juce_data_structures
  # juce::juce_events
  juce::juce_graphics
  juce::juce_gui_basics
  # juce::juce_gui_extra
  juce::juce_opengl

  elysium_rust

  PUBLIC
  juce::juce_recommended_config_flags
  juce::juce_recommended_lto_flags
  # juce::juce_recommended_warning_flags
  )

if(WIN32)
  target_link_libraries(Elysium PRIVATE
    bcrypt
    userenv)
endif()

# ==================================================

get_target_property(cargo-dir elysium_rust CARGO_DIR)
get_target_property(cargo-flags elysium_rust CARGO_FLAGS)
set(cargo-toml ${CMAKE_SOURCE_DIR}/src/rust/Cargo.toml)

file(GLOB_RECURSE cxx-sources LIST_DIRECTORIES false "${CMAKE_SOURCE_DIR}/src/*.c??")
file(GLOB_RECURSE cxx-headers LIST_DIRECTORIES false "${CMAKE_SOURCE_DIR}/src/*.h??")

add_custom_target(lint
  COMMAND clang-tidy ${cxx-sources}
  COMMAND cargo clippy ${cargo-flags} --target-dir ${cargo-dir} --manifest-path ${cargo-toml}
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  COMMENT "Running 'clang-tidy' and 'cargo clippy' on C++ and Rust sources"
  VERBATIM)

add_dependencies(lint elysium_rust)

add_custom_target(format
  COMMAND clang-format -i -style=file ${cxx-sources} ${cxx-headers}
  COMMAND cargo fmt --manifest-path ${cargo-toml}
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  COMMENT "Running 'clang-format' and 'cargo fmt' on C++ and Rust sources"
  VERBATIM)
