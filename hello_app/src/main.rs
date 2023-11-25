#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(linkage)]

static mut ABI_ADDR: usize = 0;
const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start(abi_addr: usize) -> ! {
    ABI_ADDR = abi_addr;
    hello();
    puts("Hello, world!\n");
    terminate();
    loop {}
}

fn hello() {
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            mv      a7, {abi_table}
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            abi_num = const SYS_HELLO,
            abi_table = in(reg) ABI_ADDR,
            clobber_abi("C"),
        )
    }
}

fn terminate() {
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            mv      a7, {abi_table}
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            abi_num = const SYS_TERMINATE,
            abi_table = in(reg) ABI_ADDR,
            clobber_abi("C"),
        )
    }
}

fn putchar(c: u8) {
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            mv      a7, {abi_table}
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            abi_num = const SYS_PUTCHAR,
            abi_table = in(reg) ABI_ADDR,
            in("a0") c,
            clobber_abi("C"),
        )
    }
}

fn puts(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
