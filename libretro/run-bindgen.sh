#!/bin/sh
set -ex

bindgen include/libretro.h --no-layout-tests --no-prepend-enum-name -o src/libretro_h.rs
sed -i '1i #![allow(non_camel_case_types)]' src/libretro_h.rs
sed -i '1i #![allow(non_upper_case_globals)]' src/libretro_h.rs
