// RavomIO starter for `tang_nano_9k` (framework: rust-hdl)
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
loop {}
}
