#[derive (Debug)]
struct Pair(String, String);

fn main(){
    let p = Pair("kiri".into(), "kanan".into());
    let Pair(a, b) = p;
    println!("a:{} b:{}", a,b);
    // println!("{:?}", p);
}