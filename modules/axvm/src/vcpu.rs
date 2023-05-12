use alloc::sync::Arc;
use axhal::arch;
use axsync::Mutex;

use crate::arch::vcpu::VcpuArchState;

#[derive(Default)]
pub struct VcpuState {
    pub id: u8,
    pub(crate) arch_state: VcpuArchState,
}

pub type Vcpu = Arc<Mutex<VcpuState>>;

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
    pub(crate) fn new(id: u8) -> Vcpu {
        Arc::new(Mutex::new(VcpuState {
            id,
            arch_state: VcpuArchState::default(),
        }))
    }

    pub fn run(&mut self) -> Option<VcpuExitReason> {
        loop {
            arch::disable_irqs();

            // flush interrupt
            //arch::hypervisor::flush_interrupts();

            // switch to

            // sync interrupt

            // exit
            break None;
        }
    }
}
