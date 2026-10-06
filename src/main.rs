#![no_std]
#![no_main]

mod screen;

use core::panic::PanicInfo;
use screen::colour;

core::arch::global_asm!(include_str!("boot.s"), options(att_syntax));

static M5KOS_BANNER: &[&str] = &[
    "+------------------------+",
    "|        ___ _           |",
    "|  _ __ | __| |_____ ___ |",
    "| | '  \\|__ \\ / / _ (_-< |",
    "| |_|_|_|___/_\\_\\___/__/ |",
    "|   m5ka's os v0.0.1a1   |",
    "+------------------------+",
];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(multiboot2_magic: u32, multiboot2_info_ptr: u32) -> ! {
    if multiboot2_magic != multiboot2::MAGIC {
        panic!("not multiboot2");
    }

    let multiboot2_info = unsafe {
        multiboot2::BootInformation::load(
            multiboot2_info_ptr as *const multiboot2::BootInformationHeader
        ).unwrap()
    };

    let framebuffer = multiboot2_info
        .framebuffer_tag()
        .expect("no framebuffer tag")
        .expect("unknown framebuffer type");

    let multiboot2::FramebufferType::RGB { red, green, blue } = framebuffer.buffer_type().unwrap() else {
        panic!("framebuffer is not rgb")
    };

    screen::init(screen::Framebuffer {
        addr: framebuffer.address() as *mut u32,
        width: framebuffer.width() as usize,
        height: framebuffer.height() as usize,
        pitch: framebuffer.pitch() as usize,
        red_size: red.size,
        red_shift: red.position,
        green_size: green.size,
        green_shift: green.position,
        blue_size: blue.size,
        blue_shift: blue.position,
    });

    for line in M5KOS_BANNER {
        println!("{}{}{}", colour::GREY, line, colour::RESET);
    }
    println!("hello world!");
    println!("framebuffer is {}x{} at {:#x}", framebuffer.width(), framebuffer.height(), framebuffer.address());

    loop {}
}
