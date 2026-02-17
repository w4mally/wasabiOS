#![no_std] //stdクレートは使わない
#![no_main] //main関数は使わない

#[no_mangle]
fn efi_main() {
    // println!("Hello, world!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
