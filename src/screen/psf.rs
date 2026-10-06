use include_bytes_aligned::include_bytes_aligned;

#[repr(C)]
struct Psf1Header {
    magic: u16,
    mode: u8,
    char_size: u8,
}

static FONT: &[u8] = include_bytes_aligned!(4, "../../fonts/zap-ext-light16.psf");

pub fn init() -> bool {
    if FONT.len() < size_of::<Psf1Header>() {
        return false;
    }
    let psf1_header = FONT.as_ptr().cast::<Psf1Header>();
    true
}