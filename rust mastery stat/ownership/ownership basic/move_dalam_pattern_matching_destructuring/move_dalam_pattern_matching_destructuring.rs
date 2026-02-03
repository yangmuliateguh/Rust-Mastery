#[derive(Debug)]
struct Pair(String, String);

fn main() {
    let p = Pair("kiri".into(), "kanan".into());
    let Pair(a, b) = p; // move kedua field ke `a` dan `b`
    println!("{} + {}", a, b);
    // println!("{:?}", p); // ❌ Error! `p` sudah di-destructure
}