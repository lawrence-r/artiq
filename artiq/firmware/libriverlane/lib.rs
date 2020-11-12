#![no_std]

extern crate board_misoc;
include!(concat!(env!("BUILDINC_DIRECTORY"), "/generated/csr.rs"));

#[cfg(has_hal)]
pub mod hal;

#[cfg(has_emulator)]
pub mod emulator;
