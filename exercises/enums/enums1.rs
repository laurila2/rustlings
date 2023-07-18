// enums1.rs
// No hints this time! ;)

// Quit,
// Echo(String),
// Move{ x: i32, y: i32 },
// ChangeColor(i32, i32, i32),

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
