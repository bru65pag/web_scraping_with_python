/// snake-3
/// - generate food

use sdl2::Sdl;
use sdl2::video::Window;
//use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use std::thread::sleep;
use std::time::{Duration};
//use rand::random;
use sdl2::rect::Rect;
use rand::Rng;

const VIRTUAL_WIDTH:i32 = 32; // ZX81 is 32 columns width
const VIRTUAL_HEIGHT:i32 = 24; // ZX81 is 24 lines height
const TILE_SIZE:i32 = 40;

const HEAD_COLOR:Color = Color::RGB(0,255,0);
const BODY_COLOR:Color = Color::RGB(0,0,0);

const SNAKE_SPEED:u32 = 10;

pub enum Computer {
    ZX81,
    SPECTRUM,
}

pub enum KeyPressed {
    Up,
    Down,
    Left,
    Right,
    Stop,
    Null,
}

struct Food {
    food_location:[i32;2],
}

impl Food {
    fn new(width: i32, height: i32, snake: &Snake) -> Food  {
    let mut food_x = rand::thread_rng().gen_range(0, width); // random number between 0 and x-1
    let mut food_y = rand::thread_rng().gen_range(0, height);
    // make sure it does not collide with the snake
    let mut snake_part_index=0;
    while food_x == snake.body[snake_part_index].coordinates[0]
    && food_y == snake.body[snake_part_index].coordinates[1] {
        food_x = rand::random::<i32>()* width;
        food_y = rand::random::<i32>()* height;
        snake_part_index = snake_part_index + 1;
    }
    Food {
        food_location:[food_x,food_y],
        }
    }

    // fn reset (&self, snake: &Snake) {
    //     let mut food_x = rand::random::<i32>()* self.screen_size[0];
    //     let mut food_y = rand::random::<i32>()* self.screen_size[1];
    //     // make sure it does not collide with the snake
    //     let mut snake_part_index=0;
    //     while food_x == snake.body[snake_part_index].coordinates[0]
    //     && food_y == snake.body[snake_part_index].coordinates[1] {
    //         food_x = rand::random::<i32>()* self.screen_size[0];
    //         food_y = rand::random::<i32>()* self.screen_size[1];
    //         snake_part_index = snake_part_index + 1;
    //     }
    // }
}


struct SnakePart {
    coordinates: [i32;2], // col,line
    direction: i32,
    color: Color,
}

struct Snake {
    screen_size: [i32; 2],
    body: Vec<SnakePart>,
}

impl Snake {
    fn new(width: i32, height: i32) -> Snake {
                Snake {
            screen_size:[width, height],
            body: vec![
            SnakePart { //head
                coordinates: [12,10],
                direction: 2,
                color: HEAD_COLOR,
            },
            SnakePart {
                coordinates: [11,10],
                direction: 2,
                color: BODY_COLOR,
            },
            SnakePart {
                coordinates: [10,10],
                direction: 2,
                color: BODY_COLOR,
                },
            ]
        }
    }



    /// moving the snake consists in shifting all the element of the Vector
    ///
    fn mov(&mut self, direction:i32) {
        // move body parts
        for i in (1..self.body.len()).rev() {
            self.body[i].direction = self.body[i-1].direction;
            self.body[i].coordinates = self.body[i-1].coordinates;
            self.body[i].color = BODY_COLOR;
        }
        //move head
        let mut head_x = self.body[0].coordinates[0];
        let mut head_y = self.body[0].coordinates[1];
        match direction {
            -1 => { // up
                head_y = (head_y -1 ).rem_euclid(self.screen_size[1]) ;
            },
            1 => { // down
                head_y = (head_y + 1).rem_euclid(self.screen_size[1]);
            },
            -2 => { // left
                head_x = (head_x - 1).rem_euclid(self.screen_size[0]);
            },
            2 => { // right
                head_x = (head_x + 1).rem_euclid(self.screen_size[0]);
            },
            _ => {}
        }
        self.body[0].direction = direction;
        self.body[0].coordinates[0] = head_x;
        self.body[0].coordinates[1]= head_y;
        self.body[0].color = HEAD_COLOR;
    }
}

struct Game {
    sdl: Sdl,
    canvas: Canvas<Window>,
    //key_pressed: KeyPressed,
    //board: [i32; 2],
    gameover: bool,
    direction: i32,
    //score: u32,
    snake: Snake,
    food: Food,
}

impl Game {
    /// creates a new Game instance
    /// Input paramters:
    /// x,y : location of the top left corner of the Game Window relative to screen
    fn new(x: i32, y: i32) -> Game {

        let sdl = sdl2::init().expect("failed to init SDL");
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("RUST SNAKE", (VIRTUAL_WIDTH*TILE_SIZE) as u32, (VIRTUAL_HEIGHT*TILE_SIZE) as u32)
            .position(x,y)
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        let snake = Snake::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        let food = Food::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &snake);
        Game {
            sdl,
            canvas,
            //key_pressed: KeyPressed::Null,
            //board,
            gameover: false,
            direction: 2,
            //score: 0,
            snake,
            food,
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
            self.snake.mov(self.direction);
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255,255,255));
        self.canvas.clear();
        self.draw();
        self.canvas.present();
    }

    fn draw(&mut self) {
        // draw the snake
        for i in 0..self.snake.body.len()
        {
            self.canvas.set_draw_color(self.snake.body[i].color);
            let rect = Rect::new(self.snake.body[i].coordinates[0]*TILE_SIZE+2,self.snake.body[i].coordinates[1]*TILE_SIZE+2,(TILE_SIZE-2) as u32,(TILE_SIZE-2) as u32);
            self.canvas.fill_rect(rect).unwrap();
        }
        // draw the Food
        self.canvas.set_draw_color(Color::RGB(255,0,0));
        let rect = Rect::new(self.food.food_location[0]*TILE_SIZE+2, self.food.food_location[1]*TILE_SIZE+2,(TILE_SIZE-2) as u32,(TILE_SIZE-2) as u32);
        self.canvas.fill_rect(rect).unwrap();
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
        // let dt = last_time.elapsed().as_millis();
        // println!("dt = {:?}",dt);
        // last_time = Instant::now();
        game.input();
        game.update();
        game.render();
        // Time management
        sleep(Duration::new(0, 1_000_000_000u32 / SNAKE_SPEED ));
    }
}
