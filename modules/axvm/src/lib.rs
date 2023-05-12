#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_auto_cfg)]
#![feature(asm_const, offset_of)]
#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate alloc;

pub use axhal::arch::hypervisor::{hardware_disable, hardware_enable, hypervisor_init};

pub(crate) mod arch;
pub mod mmu;
pub mod types;
pub mod vcpu;
pub mod vm;

pub fn axvm_init() {
    unsafe {
        info!("Initializing hypervisor.");
        hypervisor_init();
        hardware_enable();
    }
}

pub fn axvm_exit() {
    unsafe {
        hardware_disable();
        info!("Hypervisor Disabled.");
    }
}
