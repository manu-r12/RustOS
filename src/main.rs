#![no_std]       
#![no_main]      

use core::panic::PanicInfo;

/// This function is called when a panic occurs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Our custom entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {} // Loop forever
}