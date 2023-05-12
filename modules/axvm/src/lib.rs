#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_auto_cfg)]

#[macro_use]
extern crate log;
extern crate alloc;

pub use axhal::arch::hypervisor::{hardware_disable, hardware_enable, hypervisor_init};

pub mod mmu;
pub mod types;
pub mod vcpu;
pub mod vm;

pub fn axvm_init() {
    unsafe {
        hypervisor_init();
        hardware_enable();
    }
}

pub fn axvm_exit() {
    unsafe {
        hardware_disable();
    }
}
