extern crate sdl2;

use crate::sdl2::image::LoadTexture;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video.window("SDL Tutorial 02", 640,480)
        .position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/lannion.jpg").unwrap();
    canvas.copy(&texture,None,None).unwrap();
    canvas.present();
    sleep(Duration::from_secs(2));
}
