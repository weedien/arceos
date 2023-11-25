#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[cfg(feature = "axstd")]
use axstd::println;
#[cfg(feature = "axstd")]
use axstd::process::exit;

const PLASH_START: usize = 0x22000000;
// app running aspace
// SBI(0x80000000) -> App <- Kernel(0x80200000)
// 0xffff_ffc0_0000_0000
const RUN_START: usize = 0xffff_ffc0_8010_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let load_start = PLASH_START as *const u8;
    let load_size = 390; // 制作的bin文件中没有记录应用大小，这里手动指定

    println!("Load payload ...");

    let load_code = unsafe { core::slice::from_raw_parts(load_start, load_size) };
    println!(
        "load code {:?}; address [{:?}]",
        load_code,
        load_code.as_ptr()
    );

    let run_code = unsafe { core::slice::from_raw_parts_mut(RUN_START as *mut u8, load_size) };
    run_code.copy_from_slice(load_code);
    println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

    println!("Load payload ok!");

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_termiate as usize);

    println!("Execute app ...");

    unsafe {
        let abi_table_ptr = core::ptr::addr_of_mut!(ABI_TABLE);
        println!("ABI_TABLE Address: {:p}", abi_table_ptr);
        println!("ABI_TABLE: {:x?}", ABI_TABLE);
    }

    // execute app
    unsafe {
        core::arch::asm!("
        la      a0, {abi_table}
        li      t2, {run_start}
        jalr    t2",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
        )
    }
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_termiate() {
    println!("[ABI:Terminated] Bye!");
    exit(0);
}
