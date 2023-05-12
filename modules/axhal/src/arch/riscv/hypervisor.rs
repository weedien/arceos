use riscv::register::{hcounteren, hedeleg, hgatp, hideleg, hvip, vsie};

static mut GSTAGE_MODE: hgatp::Mode = hgatp::Mode::Bare;
static mut GSTAGE_PGD_LEVEL: usize = 0;

unsafe fn gstage_mode_detect() {
    #[cfg(target_arch = "riscv32")]
    {
        GSTAGE_MODE = hgatp::Mode::Sv32x4;
        GSTAGE_PGD_LEVEL = 2;
    }

    #[cfg(target_arch = "riscv64")]
    {
        GSTAGE_MODE = hgatp::Mode::Sv39x4;
        GSTAGE_PGD_LEVEL = 3;

        hgatp::set(hgatp::Mode::Sv57x4, 0, 0);
        if hgatp::read().mode() == hgatp::Mode::Sv57x4 {
            GSTAGE_MODE = hgatp::Mode::Sv57x4;
            GSTAGE_PGD_LEVEL = 5;
            hgatp::set(hgatp::Mode::Bare, 0, 0);
            return;
        }

        hgatp::set(hgatp::Mode::Sv48x4, 0, 0);
        if hgatp::read().mode() == hgatp::Mode::Sv48x4 {
            GSTAGE_MODE = hgatp::Mode::Sv48x4;
            GSTAGE_PGD_LEVEL = 4;
            hgatp::set(hgatp::Mode::Bare, 0, 0);
            return;
        }
    }
}

pub unsafe fn hypervisor_init() {
    gstage_mode_detect();
    info!("hgatp using {:?} G-stage page table format", GSTAGE_MODE);
}

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
