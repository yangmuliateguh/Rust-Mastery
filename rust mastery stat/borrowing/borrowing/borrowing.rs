fn main() {
    let mut s = String::from("hello");
    read_only(&s);           // pinjam immutable
    modify(&mut s);          // pinjam mutable
    println!("{}", s);       // ✅ masih milik `main`
}

fn read_only(s: &String) { println!("{}", s); }
fn modify(s: &mut String) { s.push_str("!"); }