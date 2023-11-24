#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let num = unsafe { core::slice::from_raw_parts(apps_start, 1) };
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
        println!("app{}_content: {:?}", i, code);
    }

    println!("Load payload ok!");
}

// #[inline]
// fn bytes_to_usize(bytes: &[u8]) -> usize {
//     usize::from_be_bytes(bytes.try_into().unwrap())
// }
