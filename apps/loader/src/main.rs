#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let app_size = unsafe { core::slice::from_raw_parts(apps_start, 2) };
    let app_size = (((app_size[0] as usize) << 8) + app_size[1] as usize) as usize;

    println!("app_size: {:#}", app_size);

    println!("Load payload ...");

    let start = PLASH_START + 2;
    let code = unsafe { core::slice::from_raw_parts(start as *const u8, app_size) };
    println!("content: {:?}", code);

    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}
