#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod dtb;

pub use dtb::parse_dtb;
pub use dtb::DtbInfo;
