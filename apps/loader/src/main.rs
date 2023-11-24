#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;
// app running aspace
// SBI(0x80000000) -> App <- Kernel(0x80200000)
// 0xffff_ffc0_0000_0000
const RUN_START: usize = 0xffff_ffc0_8010_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let load_start = PLASH_START as *const u8;
    let num = unsafe { core::slice::from_raw_parts(load_start, 1) };
    let apps_num = num[0] as usize;
    println!("apps_num: {:#}", apps_num);

    println!("Load payload ...");

    let mut start = PLASH_START + 1;
    for i in 0..apps_num {
        let size = unsafe { core::slice::from_raw_parts(start as *const u8, 2) };
        let app_size = (((size[0] as usize) << 8) + size[1] as usize) as usize;
        println!("app{}_size: {:#}", i, app_size);
        let content_start = (start + 2) as *const u8;
        let code = unsafe { core::slice::from_raw_parts(content_start, app_size) };
        start += 2 + app_size;
        println!(
            "app{}_content: {:?}; address [{:?}]",
            i,
            code,
            code.as_ptr()
        );

        let run_code = unsafe { core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size) };
        run_code.copy_from_slice(code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

        println!("Execute app ...");

        // execute app
        unsafe {
            core::arch::asm!("
            li t2, {run_start}
            jalr t2",
            run_start = const RUN_START,
            )
        }
    }

    println!("Load payload ok!");
}
