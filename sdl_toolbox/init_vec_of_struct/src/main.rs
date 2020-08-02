use sdl2::pixels::Color;

struct SnakePart {
  x:u32,
  y:u32,
  direction:i32,
    color: Color,
}

struct Snake {
    body: Vec<SnakePart>,
}


fn main() {

    let snake_part= SnakePart {x:1,y:1,direction:1,color:Color::RGB(0,0,0)};
    let snake = Snake {
        body: vec![SnakePart {x:1,y:1,direction:1,color:Color::RGB(0,0,0)}],
    };
}
