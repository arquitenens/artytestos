#![no_std]

use log::info;

#[panic_handler]
fn panic(reason: &core::panic::PanicInfo) -> ! {
    info!("i paniced... wow, reason: {:?}", reason);
    loop {}
    
}