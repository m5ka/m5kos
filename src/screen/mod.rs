pub mod terminal;

pub mod colour {
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[1;37m";
    pub const GREY: &str = "\x1b[1;90m";
}

pub struct Framebuffer {
    pub addr: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub red_size: u8,
    pub red_shift: u8,
    pub green_size: u8,
    pub green_shift: u8,
    pub blue_size: u8,
    pub blue_shift: u8,
}

pub fn init(framebuffer: Framebuffer) {
    terminal::init(&framebuffer);
}