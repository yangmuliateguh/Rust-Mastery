#[derive(Debug)]
struct Address {
    street: String,
}

#[derive(Debug)]
struct User {
    name: String,
    addr: Address,
}

fn main() {
    let u = User {
        name: "Budi".into(),
        addr: Address {
            street: "Jl. Merdeka".into(),
        },
    };
    take_street(u.addr.street); // move field dalam nested struct
    println!("{:?}", u); // ❌ Error! `u.addr` tidak utuh lagi
}

fn take_street(s: String) {
    println!("Jalan: {}", s);
}