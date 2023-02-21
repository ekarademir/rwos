#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rwos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rwos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rwos::test_panic_handler(info)
}

#[test_case]
// `println` works right after the boot
fn test_println() {
    println!("test_println output");
}
