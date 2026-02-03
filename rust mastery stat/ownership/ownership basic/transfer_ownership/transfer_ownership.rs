fn main() {
    let v = vec![1, 2, 3];
    take_ownership(v);        // `v` dipindahkan
    println!("{:?}", v);   // ❌ Error!
}

fn take_ownership(x: Vec<i32>) {
    println!("{:?}", x);
}