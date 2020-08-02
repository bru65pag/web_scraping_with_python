//use sdl2::pixels::PixelFormatEnum;
//use sdl2::surface::Surface;
//use sdl2::render::Canvas;
//use sdl2::video::Window;
use::sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    // we init the system
    let sdl_context = sdl2::init().expect("failed to init SDL");
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("sdl2 demo", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // draw circle
    canvas.set_draw_color(Color::RGB(255,255,255)); // white
    canvas.clear();
    canvas.circle(50,50,50,Color::RGB(255,0,0)).unwrap();
    canvas.present();
    sleep(Duration::from_secs(2));

    // draw anti alias circle
    canvas.set_draw_color(Color::RGB(255,255,255)); // white
    canvas.clear();
    canvas.aa_circle(50,50,50,Color::RGB(255,0,0)).unwrap();
    canvas.present();
    sleep(Duration::from_secs(2));

    // draw filled circleColor
    canvas.set_draw_color(Color::RGB(255,255,255)); // white
    canvas.clear();
    canvas.filled_circle(50,50,50,Color::RGB(255,0,0)).unwrap();
    canvas.present();
    sleep(Duration::from_secs(2));

    // no straight way to draw a circle with a certain line thickness
    // instead, drawing two circles
    canvas.set_draw_color(Color::RGB(255,255,255)); // white
    canvas.clear();
    canvas.filled_circle(50,50,50,Color::RGB(255,0,0)).unwrap();
    canvas.filled_circle(50,50,45,Color::RGB(255,255,255)).unwrap(); 
    canvas.present();
    sleep(Duration::from_secs(2));

}
