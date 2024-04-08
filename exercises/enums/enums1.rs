// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    // enum是一个非此即彼的类型，常常和match在一起才能体会出作用，
    // 更完美的switch！
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
