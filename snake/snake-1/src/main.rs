/// snake-1
/// - snake moves
/// - snake changes direction when player user arrow keys
/// - game exits with Esc or Window X

use sdl2::Sdl;
use sdl2::video::Window;
//use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use std::thread::sleep;
use std::time::Duration;
//use rand::random;
use sdl2::rect::Rect;

// There are multiple components to consider:
// The screen resolution: for instance 1440x900 indicate 1440 pixels width, 900 pixels height
// The computer resolution: a ZX 81 has a resolution of 24x32. With modern computers,
// the computer resolution is similar to the screen resolution
// The window resolution: it is the window in which the game is played.
// The game resolution: it could well be that the game has a map which is much bigger than the
// window in which it renders. Think of GTA for instance.
// can be a subset of the computer resolution. It is the width and height
// of the window in which the game is played
// the window resolution:
const GAME_WIDTH: i32 = 32; // ZX81 is 32 pixels width
const GAME_HEIGHT: i32 = 24; // ZX81 is 24 pixels height

const TILE_SIZE: i32 = 30; // size of a tile in pixels
const WINDOW_WIDTH:i32 = GAME_WIDTH*TILE_SIZE;
const WINDOW_HEIGHT:i32 = GAME_HEIGHT*TILE_SIZE;

const HEAD_COLOR:Color = Color::RGB(0,255,0);
const BODY_COLOR:Color = Color::RGB(0,0,0);

pub enum KeyPressed {
    Up,
    Down,
    Left,
    Right,
    Stop,
    Null,
}

struct SnakePart {
    coordinates: (i32,i32), // col,line
    direction: i32,
    color: Color,
}

struct Snake {
    body: Vec<SnakePart>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![
            SnakePart { //head
                coordinates: (12,10),
                direction: 2,
                color: HEAD_COLOR,
            },
            SnakePart {
                coordinates: (11,10),
                direction: 2,
                color: BODY_COLOR,
            },
            SnakePart {
                coordinates: (10,10),
                direction: 2,
                color: BODY_COLOR,
                },
            ]
        }
    }

    fn draw(&self, canvas: & mut Canvas<Window>,) {
        for i in 0..self.body.len()
        {
            canvas.set_draw_color(self.body[i].color);
            let rect = Rect::new(self.body[i].coordinates.0*TILE_SIZE+2,self.body[i].coordinates.1*TILE_SIZE+2,(TILE_SIZE-2) as u32,(TILE_SIZE-2) as u32);
            canvas.fill_rect(rect).unwrap();
        }
    }

    /// moving the snake consisits in shifting all the element of the Vector
    ///
    fn r#move(&mut self, direction:i32) {
        // move body parts
        for i in (1..self.body.len()).rev() {
            self.body[i].direction = self.body[i-1].direction;
            self.body[i].coordinates = self.body[i-1].coordinates;
            self.body[i].color = BODY_COLOR;
        }
        //move head
        let mut head_x = self.body[0].coordinates.0;
        let mut head_y = self.body[0].coordinates.1;
        match direction {
            -1 => { // up
                head_y = (head_y-1).rem_euclid(GAME_HEIGHT);
            },
            1 => { // down
                head_y = (head_y+1).rem_euclid(GAME_HEIGHT);
            },
            -2 => { // left
                head_x = (head_x-1).rem_euclid(GAME_WIDTH);
            },
            2 => { // right
                head_x = (head_x+1).rem_euclid(GAME_WIDTH);
            },
            _ => {}
        }
        self.body[0].direction = direction;
        self.body[0].coordinates.0 = head_x;
        self.body[0].coordinates.1 = head_y;
        self.body[0].color = HEAD_COLOR;
    }
}

struct Game {
    sdl: Sdl,
    canvas: Canvas<Window>,
    key_pressed: KeyPressed,
    board: [i32; 2],
    gameover: bool,
    direction: i32,
    score: u32,
    snake: Snake,
}

impl Game {
    /// creates a new Game instance
    /// Input paramters:
    /// x,y : location of the top left corner of the Game Window relative to screen
    fn new(x: i32, y: i32) -> Game {
        let sdl = sdl2::init().expect("failed to init SDL");
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("RUST SNAKE", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position(x,y)
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let board = [GAME_WIDTH, GAME_HEIGHT];
        let snake = Snake::new();
        Game {
            sdl,
            canvas,
            key_pressed: KeyPressed::Null,
            board,
            gameover: false,
            direction: 2,
            score: 0,
            snake,
        }
    }

    fn input(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.gameover = true;
                },
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode.unwrap() {
                    keyboard::Keycode::Escape => {
                        self.gameover = true;
                   }
                    keyboard::Keycode::Up => {
                        self.direction = -1;
                    }
                    keyboard::Keycode::Down => {
                        self.direction = 1;
                    }
                    keyboard::Keycode::Left => {
                        self.direction = -2;
                    }
                    keyboard::Keycode::Right => {
                        self.direction = 2;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }



    fn update(&mut self) {
        if !self.gameover {
            if self.direction == self.snake.body[0].direction || // direction has not changed
            self.direction == -self.snake.body[0].direction  { // direction is opposite
                self.direction = self.snake.body[0].direction; // keep previous direction
            }
            self.snake.r#move(self.direction);
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255,255,255));
        self.canvas.clear();
        self.snake.draw(&mut self.canvas);
        self.canvas.present();
    }

    // fn is_game_over(&mut self) ->bool {
    //     let head = &self.snake.body[0];
    //     for i in 1..self.snake.body.len()
    //     {
    //         if head.x == self.snake.body[i].x && head.y == self.snake.body[i].y {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
}

fn main() {
    let mut game = Game::new(100, 100);

    // game loop
    while !game.gameover {
        game.input();
        game.update();
        game.render();
        // Time management
        sleep(Duration::new(0, 1_000_000_000u32 / 10 ));
    }
}
