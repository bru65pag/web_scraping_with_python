extern crate sdl2;

use sdl2::render::TextureQuery;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::ttf::Font;
use sdl2::rect::Rect;

pub enum BackgroundColor {
    BLUE,
    RED,
    YELLOW,
    PINK,
}

struct WindowSize {
    height: u32,
    width: u32,
}

struct WindowLocation {
    x: i32,
    y: i32,
}

fn input(sdl: &sdl2::Sdl , background_color: &mut BackgroundColor, running: &mut bool) {
    let mut event_pump = sdl.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => {
                *running = false;
            }
            Event::KeyDown { timestamp: _, window_id: _,
                keycode , scancode: _, keymod: _, repeat: _ } => {
                    match keycode.unwrap() {
                        keyboard::Keycode::B => {
                            *background_color = BackgroundColor::BLUE;
                        }
                        keyboard::Keycode::R => {
                            *background_color = BackgroundColor::RED;
                        }
                        keyboard::Keycode::Y => {
                            *background_color = BackgroundColor::YELLOW;
                        }
                        keyboard::Keycode::P => {
                            *background_color = BackgroundColor::PINK;
                        }
                        keyboard::Keycode::Escape => {
                            *running = false;
                        }
                        _ => {
                        }
                    }
            }
            _ => {
            }
        }
    }
}

pub fn update() {
}

pub fn render (
    canvas: &mut Canvas<sdl2::video::Window>,
    background_color: &BackgroundColor,
    font: &Font)
    -> Result<(), String> {
// set the background color
    match background_color {
        BackgroundColor::BLUE => canvas.set_draw_color(Color::RGB(0,0,255)),
        BackgroundColor::RED => canvas.set_draw_color(Color::RGB(255,0,0)),
        BackgroundColor::YELLOW => canvas.set_draw_color(Color::RGB(255,255,0)),
        BackgroundColor::PINK  => canvas.set_draw_color(Color::RGB(255,0,255))
    }
    canvas.clear();

// render text
    let texture_creator = canvas.texture_creator();

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font.render("HELLO RUST")
        .blended(Color::RGBA(0, 0, 0, 255))
        .map_err(|e| e.to_string())?
        ;

    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?
        ;
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(50,50,width,height);
    canvas.copy(&texture, None, Some(target))?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
// read fps and # of enemies from command line
    // let args:Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     panic!("Missing fps and/or # of enemies");
    // };
    // let frames_per_second = args[1].parse::<u32>().unwrap();

// initialize
    let window_size = WindowSize {height: 800, width: 600};
    let window_location = WindowLocation {x:320, y: 300};
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Game",window_size.height,window_size.width)
        .position(window_location.x,window_location.y)
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())
    .expect("Cannot init ttf_context");
    let mut running:bool = true;
    let mut background_color = BackgroundColor::PINK;

    // Load a font
   let font = ttf_context.load_font("assets/zx81.ttf", 64)
   .expect("cannot load font");
   //font.set_style(sdl2::ttf::FontStyle::BOLD);

// game loop
    while running {
        input(&sdl, &mut background_color, &mut running);
        update();
        render(&mut canvas, &background_color, &font)?;
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
