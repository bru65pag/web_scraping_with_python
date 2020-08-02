
extern crate sdl2;

use std::convert::TryInto;
use sdl2::render::Canvas;
use std::thread::sleep;
use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::render::{WindowCanvas, Texture};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use sdl2::rect::{Rect};


fn input(sdl: &sdl2::Sdl,
    running: &mut bool) {
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
                    _ => {
                    }
                }
            }
            _ => {
            }
        }
    };
}

fn update(animations: & mut Vec<Animation>) {
    for animation in animations {
        // number of animations
        animation.current_sprite = animation.current_sprite + 1;
        if animation.current_sprite > animation.sprite_sheet.len() {
            animation.current_sprite = 0;
        }

    }
}

fn render(
    sprite: &Sprite,
    canvas: &mut WindowCanvas,
    texture: &Texture,
    animations: & Vec<Animation>)  -> Result<(), String>{
    let mut a =0;
    for anim in animations {
        // is it time to display the next sprite?

        // build a new rectangle with the next sprite to be displayed
        let src = Rect::new(
            anim.sprite_sheet[anim.current_sprite].x,
            anim.sprite_sheet[anim.current_sprite].y,
            anim.sprite_sheet[anim.current_sprite].w,
            anim.sprite_sheet[anim.current_sprite].h,
        );
        canvas.copy(texture, src, Rect::new(a,0,315,315));
        let a = a+325;
    }
    canvas.present();
    sleep(Duration::new(0, 1_000_000_000u32/60));
    Ok(())
}

pub struct Sprite {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

struct Animation {
    sprite_sheet_file_path: String,
    sprite_sheet: Vec<Sprite>,
    current_sprite: usize,
    sprite_per_sec: u32,
}

impl Animation {
    fn new(canvas: Canvas<sdl2::video::Window>, sprite_sheet_coord: &[Sprite]) -> Animation {
        let mut animation:Animation;
        for i in 0..sprite_sheet_coord.len()-1 {
            animation.sprite_sheet.push(sprite_sheet_coord[i]);
            animation.current_sprite = animation.sprite_sheet.len();
        }
    animation
    }
}

fn main() -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("chicken_run", 315*3+10+10, 315)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let animations:Vec<Animation> = Vec::new();
    let sprite_sheet_coord = [
        Sprite{x:356,y:20,w:315,h:315},
        Sprite{x:686,y:20,w:315,h:315},
        Sprite{x:26,y:348,w:315,h:315},
        Sprite{x:356,y:348,w:315,h:315},
        Sprite{x:686,y:348,w:315,h:315}
    ];
    let anim1:Animation = Animation::new(canvas,&sprite_sheet_coord);
    let anim2:Animation = Animation::new(canvas,&sprite_sheet_coord);
    let anim3:Animation = Animation::new(canvas,&sprite_sheet_coord);
    animations.push(anim1);
    animations.push(anim2);
    animations.push(anim3);

    let mut running:bool = true;
    let mut current_sprite_index:usize = 5;

    while running {
        input(&sdl_context, &mut running);
        update(&mut animations);
        let a_sprite = &chicken_sprite_sheet[current_sprite_index];
        render(a_sprite, &mut canvas, &texture, &animations)?;
        // Time management
        sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }

    Ok(())
}
