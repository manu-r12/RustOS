#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// This function is the entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        *vga_buffer.offset(0) = b'H';
        *vga_buffer.offset(1) = 0x0f;
        *vga_buffer.offset(2) = b'e';
        *vga_buffer.offset(3) = 0x0f;
        *vga_buffer.offset(4) = b'l';
        *vga_buffer.offset(5) = 0x0f;
        *vga_buffer.offset(6) = b'l';
        *vga_buffer.offset(7) = 0x0f;
        *vga_buffer.offset(8) = b'o';
        *vga_buffer.offset(9) = 0x0f;
    }

    loop {}
}