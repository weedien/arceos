use riscv::register::{hcounteren, hedeleg, hideleg, hvip, vsie};

//#[cfg(target_arch = "riscv32")]
//static GSTAGE_MODE: hgatp::Mode = hgatp::Mode::Sv32x4;
//#[cfg(target_arch = "riscv64")]
//static GSTAGE_MODE: hgatp::Mode = hgatp::Mode::Sv39x4;
//#[cfg(target_arch = "riscv32")]
//static GSTAGE_PGD_LEVEL: usize = 2;
//#[cfg(target_arch = "riscv64")]
//static GSTAGE_PGD_LEVEL: usize = 3;

pub unsafe fn hypervisor_init() {}

pub unsafe fn hardware_enable() {
    hedeleg::clear();
    hedeleg::set_instruction_misaligned();
    hedeleg::set_breakpoint();
    hedeleg::set_user_env_call();
    hedeleg::set_instruction_page_fault();
    hedeleg::set_load_page_fault();
    hedeleg::set_store_page_fault();

    hideleg::clear();
    hideleg::set_vsoft();
    hideleg::set_vtimer();
    hideleg::set_vext();

    hcounteren::clear();
    hcounteren::set_tm();

    hvip::clear();
}

pub unsafe fn hardware_disable() {
    vsie::clear();
    hvip::clear();
    hedeleg::clear();
    hideleg::clear();
}
