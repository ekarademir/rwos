#![no_std]
#![no_main]

mod vga_buffer;

use core::{fmt::Write, panic::PanicInfo};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again!").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} and {}",
        42,
        22.0 / 7.0
    )
    .unwrap();
    loop {}
}
