mod graphics;
mod terminal;
mod psf;

pub fn init() {
    graphics::init();
    terminal::init();
}