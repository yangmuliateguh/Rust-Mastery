fn main() {
    let s = String::from("dari luar");
    let closure = move || {
        println!("{}", s); // `s` dipindahkan ke closure
    };
    closure();
    // println!("{}", s); // ❌ Error!
}