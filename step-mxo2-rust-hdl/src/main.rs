// RavomIO starter for `step_mxo2_lpc` (framework: rust-hdl)
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
