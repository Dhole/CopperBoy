#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::identity_op)]
#![allow(clippy::upper_case_acronyms)]
// #![no_std]

// #[cfg(feature = "std")]
// use log;

use core::ops::{Index, IndexMut};

#[macro_use]
extern crate num_derive;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "logging")]
macro_rules! debug {
    ($($arg:tt)+) => (
        log::debug!($($arg)+)
    )
}

#[cfg(not(feature = "logging"))]
macro_rules! debug {
    ($($arg:tt)+) => {
        ()
    };
}

#[cfg(feature = "logging")]
macro_rules! warn {
    ($($arg:tt)+) => (
        log::warn!($($arg)+)
    )
}

#[cfg(not(feature = "logging"))]
macro_rules! warn {
    ($($arg:tt)+) => {
        ()
    };
}

#[cfg(feature = "logging")]
macro_rules! info {
    ($($arg:tt)+) => (
        log::info!($($arg)+)
    )
}

#[cfg(not(feature = "logging"))]
macro_rules! info {
    ($($arg:tt)+) => {
        ()
    };
}

#[cfg(not(feature = "vec_unchecked"))]
macro_rules! vec_get {
    ($vec:expr, $index:expr) => {
        $vec[$index]
    };
}

#[cfg(feature = "vec_unchecked")]
macro_rules! vec_get {
    ($vec:expr, $index:expr) => {
        unsafe { *$vec.get_unchecked($index) }
    };
}

#[cfg(not(feature = "vec_unchecked"))]
macro_rules! vec_set {
    ($vec:expr, $index:expr, $value:expr) => {
        $vec[$index] = $value
    };
}

#[cfg(feature = "vec_unchecked")]
macro_rules! vec_set {
    ($vec:expr, $index:expr, $value:expr) => {
        unsafe { *$vec.get_unchecked_mut($index) = $value }
    };
}

// #[derive(PartialEq, Debug)]
// struct Memory {
//     buf: Vec<u8>,
// }
//
// impl Memory {
//     fn new(size: u16) -> Self {
//         Self {
//             buf: vec![0; size as usize],
//         }
//     }
//
//     fn get_u16(&mut self, a: u16) -> u16 {
//         u16::from_le_bytes([self.buf[a as usize], self.buf[(a + 1) as usize]])
//     }
//
//     fn set_u16(&mut self, a: u16, v: u16) {
//         let bytes = v.to_le_bytes();
//         self.buf[a as usize] = bytes[0];
//         self.buf[(a + 1) as usize] = bytes[1];
//     }
// }
//
// impl Index<u16> for Memory {
//     type Output = u8;
//
//     fn index(&self, i: u16) -> &u8 {
//         &self.buf[i as usize]
//     }
// }
//
// impl IndexMut<u16> for Memory {
//     fn index_mut(&mut self, i: u16) -> &mut u8 {
//         &mut self.buf[i as usize]
//     }
// }

pub mod clock;
pub mod display;
pub mod eeprom;
pub mod emulator;
pub mod int_vec;
pub mod io_regs;
pub mod keys;
pub mod mcu;
pub mod opcodes;
pub mod utils;
