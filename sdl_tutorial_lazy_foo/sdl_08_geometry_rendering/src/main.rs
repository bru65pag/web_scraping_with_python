use std::thread::sleep;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;

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

    // clear screen
    canvas.set_draw_color(Color::RGB(255,255,255)); // white
    canvas.clear(); // clear the canvas with  the color set on the previous line

    // red fill_rectangle
    canvas.set_draw_color(Color::RGB(255,0,0));
    let fill_rectangle = Rect::new((SCREEN_WIDTH/4) as i32,(SCREEN_HEIGHT/4) as i32,SCREEN_WIDTH/2 as u32,SCREEN_HEIGHT/2);
    canvas.fill_rect(fill_rectangle).unwrap();

    // green outline rectangle
    canvas.set_draw_color(Color::RGB(0,255,0));
    let outline_rectangle = Rect::new((SCREEN_WIDTH/6) as i32,(SCREEN_HEIGHT/6) as i32,SCREEN_WIDTH*2/3 ,SCREEN_HEIGHT*2/3);
    canvas.draw_rect(outline_rectangle).unwrap();

    // blue horizontal line
    canvas.set_draw_color(Color::RGB(0,0,255));
    let p1 = Point::new(0,(SCREEN_HEIGHT/2) as i32);
    let p2 = Point::new(SCREEN_WIDTH as i32,(SCREEN_HEIGHT/2) as i32);
    canvas.draw_line(p1,p2).unwrap();

    // vertical yellow line
    canvas.set_draw_color(Color::RGB(255,255,0));
    for i in (0..SCREEN_HEIGHT).step_by(4) {
        let p1 = Point::new((SCREEN_WIDTH/2) as i32,i as i32);
        canvas.draw_point(p1).unwrap();
    }
    canvas.present();

    sleep(Duration::from_secs(5));
}
