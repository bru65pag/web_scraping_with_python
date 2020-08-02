use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub fn main() {
    // we init the system
    let sdl_context = sdl2::init().expect("failed to init SDL");
    let video_subsystem = sdl_context.video().expect("failed to get video context");

    let window = video_subsystem.window("sdl2 demo", 800, 600)
        .build()
        .expect("failed to build window");
    let surface = Surface::load_bmp("assets/foo.bmp").unwrap();
    let mut canvas: Canvas<Surface> = surface.into_canvas().unwrap();
    canvas.present();


}
