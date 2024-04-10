// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit = 12,
    Echo = 34,
    Move = 36,
    ChangeColor = 11
}

fn main() {
    println!("{:?}", Message::Quit as u32);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
