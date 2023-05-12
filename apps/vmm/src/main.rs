#![no_std]
#![no_main]

#[macro_use]
extern crate libax;
extern crate alloc;

use libax::task;
use libax::time::Duration;
use libax::vmm::mmu::{GuestPhysAddr, MappingFlags, MemMap};
use libax::vmm::vcpu::Vcpu;
use libax::vmm::vm::VmState;

const NUM_VCPU: u8 = 4;

fn vcpu_main(vcpu: Vcpu) {
    loop {
        if let Some(_) = vcpu.lock().run() {
            break;
        }
        break;
    }
}

#[no_mangle]
fn main() {
    let mut vm = match VmState::new(NUM_VCPU) {
        Ok(vm) => vm,
        Err(e) => {
            error!("Creating VM failed: {}", e.as_str());
            return;
        }
    };

    info!("Creating VM success.");

    let memmap = match MemMap::alloc(
        GuestPhysAddr::from(0x8000_0000),
        16 << 20,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
    ) {
        Ok(memmap) => memmap,
        Err(e) => {
            error!("Creating Memory Mapping failed: {}", e.as_str());
            return;
        }
    };

    vm.create_memmap(memmap);

    info!("Creating MemMap success.");

    for i in 0..NUM_VCPU {
        let vcpu = vm.create_vcpu(i);
        task::spawn(move || {
            vcpu_main(vcpu);
        });
    }

    println!("sleep for 10s");

    task::sleep(Duration::from_secs(10));
}
