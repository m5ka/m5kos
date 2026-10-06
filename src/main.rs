#![no_std]
#![no_main]

mod screen;

use core::panic::PanicInfo;

core::arch::global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_mb_magic: u32, _mb_info: u32) -> ! {
    screen::init();
    loop {}
}
