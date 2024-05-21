// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn find_area(rect: Rectangle) -> f64 {

    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;

    return width * height;
}

// simple enum
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// enum with data
enum Message {
    Quit,                       // No associated data
    Move { x: i32, y: i32 },    // Named fields
    Write(String),              // Tuple struct variant
    ChangeColor(i32, i32, i32), // Tuple variant with three i32 values
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => {
                println!("Moving in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to red {}, green {}, and blue {}", r, g, b);
            }
        }
    }

    fn new_write_message(text: &str) -> Message {
        Message::Write(text.to_string())
    }
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn main() {

    let name = "Peter";
    let age = 27;
    let peter = Person {
        name: name.to_string(),
        age: age,
    };

    println!("{:?}", peter);

    let p1: Point = Point { x: 1.0, y: 4.0 };
    let p2: Point = Point { x: 3.0, y: 2.0 };
    let rect: Rectangle = Rectangle { top_left: p1, bottom_right: p2 };

    println!("rectangle: {:?}", rect);
    let area: f64 = find_area(rect);
    println!("rectangle: {}", area);

    // enums
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    println!("{:?}", up);
    println!("{:?}", down);
    println!("{:?}", left);
    println!("{:?}", right);

    let quit = Message::Quit;
    let mov = Message::Move { x: 1, y: 2 };
    let write = Message::new_write_message("Hello, world");
    let change_color = Message::ChangeColor(0, 160, 255);

    quit.call();
    mov.call();
    write.call();
    change_color.call();

    // constants
    println!("{}", THRESHOLD);
    println!("{}", LANGUAGE);
}