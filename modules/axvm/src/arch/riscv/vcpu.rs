use axhal::GeneralRegisters;

#[derive(Debug, Default)]
pub(crate) struct CpuContext {
    pub regs: GeneralRegisters,
    pub sepc: usize,
    pub sstatus: usize,
    pub hstatus: usize,
}

#[derive(Debug, Default)]
pub(crate) struct VcpuCsr {
    pub vsstatus: usize,
    pub vsie: usize,
    pub vstvec: usize,
    pub vsscratch: usize,
    pub vsepc: usize,
    pub vscause: usize,
    pub vstval: usize,
    pub hvip: usize,
    pub vsatp: usize,
    pub scounteren: usize,
}

#[derive(Debug, Default)]
pub struct VcpuArchState {
    pub mvendorid: usize,
    pub marchid: usize,
    pub mimpid: usize,

    pub host_sscratch: usize,
    pub host_stvec: usize,
    pub host_scounteren: usize,

    pub host_context: CpuContext,
    pub guest_context: CpuContext,
    pub guest_csr: VcpuCsr,
}

