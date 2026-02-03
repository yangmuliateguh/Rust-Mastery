#[derive(Debug)]
enum Message {
    Text(String),
    Code(u32),
}

fn main() {
    let m = Message::Text("penting".into());
    if let Message::Text(content) = m {
        use_text(content); // `content` dipindahkan
    }
    // println!("{:?}", m); // ❌ Error! `m` sudah dipindahkan
}

fn use_text(s: String) {
    println!("Isi: {}", s);
}