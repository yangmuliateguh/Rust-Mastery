fn main(){
    // STACK
    let x: i32 = 20;
    println!("x: {:p}", &x);
    let y = x;
    println!("y: {:p}", &y);
    let s1: String = String::from("hello");
    println!("s1: {:p}", &s1);
    // HEAP
    println!("s1: {:p}", s1.as_ptr());
}