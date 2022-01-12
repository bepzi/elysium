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

set(cxx_clang $<CXX_COMPILER_ID:Clang,AppleClang,IntelClang,ARMClang>)
set(cxx_gcc $<CXX_COMPILER_ID:GNU>)
set(cxx_msvc $<CXX_COMPILER_ID:MSVC>)

if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "AppleClang")
  set(cxx_compiler_warnings "-Wall;-Wextra;-Wconversion;-Wfloat-equal;-Wshadow;-Werror=format-security;-Wpedantic;-pedantic;-pedantic-errors;")
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  set(cxx_compiler_warnings "${cxx_compiler_warnings}-Woverride;")
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "AppleClang")
  set(cxx_compiler_warnings "${cxx_compiler_warnings}-Wmost;-Wheader-hygiene;-Widiomatic-parentheses;-Wmove;-Wloop-analysis;")
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(cxx_compiler_warnings "/W4;/GS;")
endif()
