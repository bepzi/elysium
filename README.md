<h2 align="center">Elysium</h2>

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://opensource.org/licenses/GPL-3.0)

A work-in-progress JUCE app, powered by Rust. It can be used as a
standalone application or as an LV2 plugin. Developed on Linux, and
tested in Ardour and Carla.

### Building

You'll want `cargo`, `clang-format`, and `clang-tidy` in your `$PATH`.

```bash
cmake -S . -B build
cmake --build build --config RelWithDebInfo -j$(nproc)
```

Available targets passable to `cmake` as `--target <target_name>` are:
`Elysium_Standalone`, `Elysium_LV2`, `lint`, and `format`.
