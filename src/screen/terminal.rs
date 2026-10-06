use core::ffi::c_void;
use core::fmt::{self, Write};
use core::ptr::null_mut;
use flanterm::sys::{flanterm_context, flanterm_fb_init, flanterm_write};
use include_bytes_aligned::include_bytes_aligned;
use spin::Mutex;

struct Terminal(*mut flanterm_context);
unsafe impl Send for Terminal {}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { flanterm_write(self.0, s.as_ptr().cast(), s.len()) };
        Ok(())
    }
}

static TERMINAL: Mutex<Option<Terminal>> = Mutex::new(None);

static FONT: &[u8] = include_bytes_aligned!(4, "../../fonts/zap-ext-light16.psf");

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    if let Some(term) = TERMINAL.lock().as_mut() {
        let _ = term.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::screen::terminal::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

pub fn init(framebuffer: &super::Framebuffer) {
    let ft_context = unsafe {
        flanterm_fb_init(
            None, None,
            framebuffer.addr, framebuffer.width, framebuffer.height, framebuffer.pitch,
            framebuffer.red_size, framebuffer.red_shift,
            framebuffer.green_size, framebuffer.green_shift,
            framebuffer.blue_size, framebuffer.blue_shift,
            null_mut(),
            null_mut(), null_mut(), null_mut(), null_mut(), null_mut(), null_mut(),
            FONT[4..].as_ptr() as *mut c_void, 8, FONT[3] as usize, 1,
            1, 1,
            0,
        )
    };
    *TERMINAL.lock() = Some(Terminal(ft_context));
}