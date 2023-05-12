use alloc::sync::Arc;

use crate::vm::VmState;

#[derive(Debug, Default)]
pub struct VcpuState {
    pub id: u8,
}

pub enum VcpuExitReason {
    Unknown,
    Exception,
    Io,
    Mmio,
    Hlt,
    #[cfg(target_arch = "x86_64")]
    Rdmsr,
    #[cfg(target_arch = "x86_64")]
    Wrmsr,
    #[cfg(target_arch = "aarch64")]
    NISV,
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    SBI,
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    CSR,
}

impl VcpuState {
    pub fn new(id: u8, vm: Arc<VmState>) -> Arc<Self> {
        let vcpu = Arc::new(VcpuState { id });
        vm.create_vcpu(vcpu.clone());
        vcpu
    }

    pub fn reset(&mut self) {}

    pub fn run(&self) -> Option<usize> {
        None
    }
}
