fn categorize(code: u16) -> String {
    match code {
        200 | 201 | 202 => "Success".into(),
        400..=499 => format!("Client Error: {}", code),
        500..=599 => format!("Server Error: {}", code),
        other => format!("Unknown Code: {}", other)
    }
}

fn main() {
    println!("{}", categorize(200));
    println!("{}", categorize(400));
    println!("{}", categorize(500));
    println!("{}", categorize(999));
}