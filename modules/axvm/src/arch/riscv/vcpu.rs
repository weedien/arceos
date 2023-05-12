use core::arch::global_asm;
use core::mem::offset_of;

use axhal::arch::GeneralRegisters;

#[repr(C)]
#[derive(Default)]
pub(crate) struct CpuContext {
    pub regs: GeneralRegisters,
    pub sepc: usize,
    pub sstatus: usize,
    pub hstatus: usize,
}

#[repr(C)]
#[derive(Default)]
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

#[repr(C)]
pub(crate) struct VcpuArchState {
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

impl Default for VcpuArchState {
    fn default() -> Self {
        let mvendorid = sbi_rt::get_mvendorid();
        let marchid = sbi_rt::get_marchid();
        let mimpid = sbi_rt::get_sbi_impl_id();
        // SPP & SPIE
        // from S-mode with interrupt enabled
        let sstatus = 0x00000120;
        // SPV & SPVP && VTW
        // to VS-mode with WFI trapping enabled
        let hstatus = 0x00200180;
        let guest_context = CpuContext {
            regs: GeneralRegisters::default(),
            sepc: 0,
            sstatus,
            hstatus,
        };

        Self {
            mvendorid,
            marchid,
            mimpid,
            host_sscratch: 0,
            host_stvec: 0,
            host_scounteren: 0,
            host_context: CpuContext::default(),
            guest_context,
            guest_csr: VcpuCsr::default(),
        }
    }
}

include_asm_marcos!();

global_asm!("
.global context_switch
context_switch:

    REG_S   t0, a0, {mvendorid}
    REG_L   t0, a0, {vsatp}
    ",
    mvendorid = const offset_of!(VcpuArchState, mvendorid),
    vsatp = const offset_of!(VcpuArchState, guest_csr.vsatp),
);
