extern crate sdl2;

use std::thread::sleep;
use std::convert::TryFrom;
use sdl2::render::TextureQuery;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::pixels::Color;
//use std::env;
use std::time::Duration;
use sdl2::ttf::Font;
use sdl2::rect::Rect;


pub struct Timer {
    sdl2_timer: sdl2::TimerSubsystem,
    is_started: bool,
    is_paused: bool,
    start_ticks: u32,    // value of tick when timer starts
    paused_ticks: u32, // value of tick when time is paused
}


impl Timer {

    fn new(sdl_context: &sdl2::Sdl) -> Timer {
        Timer {
            sdl2_timer: sdl_context.timer().unwrap(),
            is_started: false,
            is_paused: false,
            start_ticks: 0,
            paused_ticks: 0,
        }
    }

    fn start(self: &mut Timer) {
        self.is_started = true;
        self.is_paused = false;
        self.start_ticks = self.sdl2_timer.ticks();
        self.paused_ticks = 0;
    }

    fn stop(self: &mut Timer) {
        self.is_started = false;
        self.is_paused = false;
        self.start_ticks = 0;
        self.paused_ticks = 0;
    }

    fn pause(self: &mut Timer) {
        if self.is_started & !self.is_paused { // timer is started and isn't paused
            self.is_paused = true;
            self.paused_ticks = self.sdl2_timer.ticks() - self.start_ticks;
            self.start_ticks = 0;
        }
    }

    fn unpause(self: &mut Timer) {
        if self.is_started & self.is_paused { // timer is started and paused
            self.is_paused = false;
            self.start_ticks =  self.sdl2_timer.ticks() - self.paused_ticks;
            self.paused_ticks = 0;
        }
    }

    fn get_ticks(self: &mut Timer) -> u32 {
        if self.is_started {
            if self.is_paused {
                self.paused_ticks
            }
            else {
                self.sdl2_timer.ticks() - self.start_ticks
            }
        } else {
            0
        }
    }

    fn is_started (self: &Timer) -> bool {
        self.is_started
    }

    fn is_paused (self: &Timer) -> bool {
        self.is_started & self.is_paused
    }
}

pub enum BackgroundColor {
    BLUE,
    RED,
    YELLOW,
    PINK,
}

pub enum KeyPressed {
    Pause,
    StartStop,
    Null,
}

struct WindowSize {
    height: u32,
    width: u32,
}

struct WindowLocation {
    x: i32,
    y: i32,
}

fn input(sdl: &sdl2::Sdl,
    running: &mut bool,
    background_color: &mut BackgroundColor) {
    let mut event_pump = sdl.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => {
                *running = false;
            }
            Event::KeyDown { timestamp: _, window_id: _,
                keycode , scancode: _, keymod: _, repeat: _ } => {
                match keycode.unwrap() {
                    keyboard::Keycode::Escape => {
                        *running = false;
                    }
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
                    _ => {
                    }
                }
            }
            _ => {
            }
        }
    }
}

pub fn update(fps_timer: &mut Timer, counted_frames: &mut f64, average_fps: &mut f64) {
    *average_fps = *counted_frames/(fps_timer.get_ticks() as f64/1_000.0);
    if *average_fps > 2_000_000.0 {
        *average_fps = 0.0;
    }
}

pub fn render (
    canvas: &mut Canvas<sdl2::video::Window>,
    font: &Font,
    background_color: &BackgroundColor,
    average_fps: &mut f64)
    -> Result<(), String> {

    match background_color {
        BackgroundColor::BLUE => canvas.set_draw_color(Color::RGB(0,0,255)),
        BackgroundColor::RED => canvas.set_draw_color(Color::RGB(255,0,0)),
        BackgroundColor::YELLOW => canvas.set_draw_color(Color::RGB(255,255,0)),
        BackgroundColor::PINK  => canvas.set_draw_color(Color::RGB(255,0,255))
    }
    canvas.clear();

// print header text
    let texture_creator = canvas.texture_creator();
    let surface = font.render("SDL_24_CALCULATING_FRAME_RATE")
        .blended(Color::RGBA(0, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    let (canvas_width, _canvas_height) = canvas.output_size()?;
    let target = Rect::new(i32::try_from((canvas_width-width)/2).unwrap(),10,width,height);
    canvas.copy(&texture, None, Some(target))?;

// print frame rate
    let text1 = "Average Frames per second: ".to_string();
    let text2 = format!("{:.2}",&average_fps);
    let text = text1 + &text2;
    let surface = font.render(&text)
        .blended(Color::RGBA(0, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(50,200,width,height);
    canvas.fill_rect(Rect::new(50, 200,500,29)).unwrap();
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

    let window_size = WindowSize {height: 800, width: 400};
    let window_location = WindowLocation {x:320, y: 300};
    let sdl_context = sdl2::init().unwrap();

    let video_context = sdl_context.video().unwrap();
    let window_context = video_context.window("sdl_24_calculating_frame_rate",window_size.height,window_size.width)
        .position(window_location.x,window_location.y)
        //.resizable()
        .build()
        .unwrap();
    let mut canvas_context = window_context.into_canvas()
        .present_vsync() // if commented out, fps value close to 578. in not, around 60 which is the refresh rate of the screeen.
        .build()
        .unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())
    .expect("Cannot init ttf_context");

    let mut running:bool = true;
    let mut background_color = BackgroundColor::PINK;

    let mut fps_timer = Timer::new(&sdl_context);
    let mut counted_frames:f64 = 0.0;
    let mut average_fps:f64 = 0.0;
    fps_timer.start();


    // Load a font
   let font = ttf_context.load_font("assets/basic_sans_serif_7.ttf", 32)
   .expect("cannot load font");
   //font.set_style(sdl2::ttf::FontStyle::BOLD);

// game loop
    while running {
        input(&sdl_context, &mut running, &mut background_color);
        update(&mut fps_timer,&mut counted_frames, &mut average_fps);
        render(&mut canvas_context, &font, &background_color, &mut average_fps)?;
        // Time management
        //sleep(Duration::new(0, 1_000_000_000u32 / 20));
        counted_frames += 1.0;
    }

    Ok(())
}
