#![no_std]

extern crate board_misoc;
include!(concat!(env!("BUILDINC_DIRECTORY"), "/generated/csr.rs"));

#[cfg(has_rabi)]
pub mod rabi;
