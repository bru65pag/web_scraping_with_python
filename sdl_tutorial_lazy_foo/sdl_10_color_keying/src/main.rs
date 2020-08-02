// 13_alpha_blending

extern crate sdl2;

use sdl2::video::WindowContext;
use sdl2::render::BlendMode;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::{TextureCreator, Texture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::path::Path;
use crate::sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

struct LTexture<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl LTexture<'_> {
    pub fn new<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> LTexture<'a> {

        let texture = texture_creator.load_texture(path)
                .expect("cannot initialize texture");
        let w = texture.query().width;
        let h = texture.query().height;

        LTexture {
            texture: texture,
            width: w,
            height: h,
        }
    }
    pub fn set_blend_mode(&mut self, blend_mode:BlendMode) {
        self.texture.set_blend_mode(blend_mode);
    }

    // Set the alpha channel of the texture, controlling its transparency
    fn set_alpha(&mut self, alpha: u8) {
        self.texture.set_alpha_mod(alpha);
    }

    fn render_to (&mut self,canvas: &mut Canvas<Window>, x:i32, y:i32) {
        let rect = Rect::new(0, 0, self.width, self.height);
        canvas.copy(&self.texture,rect, Rect::new(x, y,rect.width(),rect.height()))
            .unwrap();
    }
}
/// creates a window with a canvas, set the background color
/// different uses of Rect on a canvas
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // create the window
    let window = video.window("sdl_tutorial_001",640,480)
        .position_centered()
        //.resizable()
        .build()
        .expect("could not initialize window context");

    let mut canvas = window.into_canvas().build()
        .expect("could not initialize canvas");

    let texture_creator = canvas.texture_creator();

    let mut background_texture = LTexture::new(&texture_creator, Path::new("assets/background.png"));

    let mut foo_texture = LTexture::new(&texture_creator, Path::new("assets/foo.png"));

    let mut running: bool = true;

    // Get a handle to the SDL2 event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Set the current alpha to max (255).
    let mut alpha: u8 = 0xff;

    // game loop
    while running {
        // Extract any pending events from from the event pump and process them
        for event in event_pump.poll_iter() {
            // pattern match on the type of event
            match event {
                Event::Quit {..} => {
                    running = false
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0x0, 0x0, 0x0));
        canvas.present();

        // Set the alpha on the modulated texture
        modulated_texture.set_alpha(alpha);

        // render the background texture
        background_texture.render_to(&mut canvas, 0, 0);

        // render Foo
        modulated_texture.render_to(&mut canvas, 240, 190);

        // Update the screen
        canvas.present();
    }
}
