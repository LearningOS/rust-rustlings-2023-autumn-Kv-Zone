// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    //定义下面用的元素
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
