use sdl2::image::LoadTexture;
use std::thread::sleep;
use std::time::Duration;
use sdl2::rect::Rect;

/// Rust version of http://lazyfoo.net/tutorials/SDL/08_geometry_rendering/index.php

fn main() {
    let SCREEN_HEIGHT:u32 = 480;
    let SCREEN_WIDTH:u32 = 640;

    let sdl_context = sdl2::init()
        .expect("could not initialize sdl context");

    let video_context = sdl_context.video()
        .expect("could not initialize video context");

    let window_context = video_context.window("sdl_08_geometry_rendering",SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        //.resizable()
        .build()
        .expect("could not initialize window context");

    let mut canvas = window_context.into_canvas().build()
        .expect("could not initialize canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/viewport.png")
        .expect("cannot initialize texture");

    // viewport 1
    let viewport1 = Rect::new(0,0,SCREEN_WIDTH/2,SCREEN_HEIGHT/2);
    canvas.set_viewport(viewport1);
    canvas.copy(&texture,None,None).unwrap();

    let viewport2 = Rect::new((SCREEN_WIDTH/2) as i32,0,SCREEN_WIDTH/2,SCREEN_HEIGHT/2);
    canvas.set_viewport(viewport2);
    canvas.copy(&texture,None,None).unwrap();

    let viewport3 = Rect::new(0,(SCREEN_HEIGHT/2) as i32,SCREEN_WIDTH,SCREEN_HEIGHT/2);
    canvas.set_viewport(viewport3);
    canvas.copy(&texture,None,None).unwrap();

    canvas.present();

    sleep(Duration::from_secs(5));
}
