#![no_std]
#![no_main]

#[macro_use]
extern crate libax;
extern crate alloc;

use alloc::sync::Arc;
use libax::task;
use libax::time::Duration;
use libax::vmm::{VcpuState, VmState};

const NUM_VCPU: u8 = 4;

fn vcpu_main(vcpu: Arc<VcpuState>) {}

#[no_mangle]
fn main() {
    let vm = Arc::new({
        match VmState::new(NUM_VCPU) {
            Ok(vm) => vm,
            Err(e) => {
                error!("Creating VM failed: {}", e.as_str());
                return;
            }
        }
    });
    for i in 0..NUM_VCPU {
        let vcpu = VcpuState::new(i, vm.clone());
        task::spawn(move || {
            vcpu_main(vcpu);
        });
    }

    println!("sleep for 60s");
    task::sleep(Duration::from_secs(60));
}
