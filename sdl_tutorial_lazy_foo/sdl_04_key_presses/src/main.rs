//extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// creates a window with a canvas, set the background color
/// different uses of Rect on a canvas
fn main() {
    let sdl_context = sdl2::init()
        .expect("could not initialize sdl context");
    let mut event_pump = sdl_context.event_pump()
        .expect("could not initialize event pump");

    let video_context = sdl_context.video()
        .expect("could not initialize video context");

    let window_context = video_context.window("sdl_04_key_presses",640,480)
        .position_centered()
        //.resizable()
        .build()
        .expect("could not initialize window context");

    let mut canvas = window_context.into_canvas().build()
        .expect("could not initialize canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.load_texture("assets/press.bmp")
            .expect("cannot initialize texture");

    // loop until Esc or Quit
    'running: loop {
        // process input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    texture = texture_creator.load_texture("assets/left.bmp")
                        .expect("cannot initialize texture");
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    texture = texture_creator.load_texture("assets/right.bmp")
                        .expect("cannot initialize texture");
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    texture = texture_creator.load_texture("assets/up.bmp")
                        .expect("cannot initialize texture");
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    texture = texture_creator.load_texture("assets/down.bmp")
                        .expect("cannot initialize texture");
                }
                _ => {}
            }
            canvas.copy(&texture,None,None).unwrap();
            canvas.present();
        }
    }
}
