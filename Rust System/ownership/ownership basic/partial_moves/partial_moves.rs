#[derive (Debug)]

struct User {
    name: String,
    age: u8
}

fn take_name(n: String) -> String { n }

fn main() {
    let u = User {
        name: "joko".into(),
        age: 30
    }
    take_name(u.name);
    println!("{:?}", u);
}