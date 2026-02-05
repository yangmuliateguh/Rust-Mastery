enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn handle(msg: Message) -> String {
    match msg {
        Message::Quit => "quitting . . .".into(),
        Message::Move {x,y} => format!("moving: {}, {}", x, y),
        Message::Write(text) => format!("{}", text),
        Message::ChangeColor(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
    }
}

fn main(){
    let quit = Message::Quit;
    let _move = Message::Move{x: 2, y:1};
    let write = Message::Write("asikjuga".into());
    let change_color = Message::ChangeColor(255, 255, 255);

    println!("{}", handle(write));
    println!("{}", handle(change_color));
    println!("{}", handle(_move));
    println!("{}", handle(quit));
}