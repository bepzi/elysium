juce_add_plugin(elysium_cpp
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

set_target_properties(elysium_cpp PROPERTIES
  CXX_STANDARD 20
  CXX_STANDARD_REQUIRED ON
  CXX_EXTENSIONS OFF)

set(Elysium_sources
  elysium.cpp
  elysium.hpp
  macros.hpp
  owning_mutex.hpp)

target_sources(elysium_cpp PUBLIC ${Elysium_sources})
set_source_files_properties(${Elysium_sources}
  PROPERTIES
  COMPILE_OPTIONS
  ${cxx_compiler_warnings})

target_include_directories(elysium_cpp PUBLIC
  .)

target_compile_definitions(elysium_cpp PUBLIC
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

target_link_libraries(elysium_cpp
  PRIVATE
  juce::juce_audio_basics
  juce::juce_audio_devices
  juce::juce_audio_plugin_client
  juce::juce_audio_processors
  juce::juce_audio_utils
  juce::juce_core
  juce::juce_graphics
  juce::juce_gui_basics
  juce::juce_opengl

  elysium_rust

  PUBLIC
  juce::juce_recommended_config_flags
  juce::juce_recommended_lto_flags)

if(WIN32)
  target_link_libraries(elysium_cpp PRIVATE
    bcrypt
    userenv)
endif()
