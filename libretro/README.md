# libretro Rust package

platform-agnostic libretro implementation of the `arduboy` emulator.  This library is not intended
to be built stand-alone but instead used as a dependency from another library to expose its
functions for a platform build.

Types for libretro functions are generated from the `include/libretro.h` header with bindgen (and
some tweaks) by running the `run-bindgen.sh` script.
