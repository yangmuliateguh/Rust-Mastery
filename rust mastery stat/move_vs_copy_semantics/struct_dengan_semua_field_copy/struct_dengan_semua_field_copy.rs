#[derive (Clone, Copy)]
struct Point{
    x: i32,
    y: i32,
}

fn main(){
    let p1 = Point {
        x: 20,
        y: 30
    };
    let p2 = p1;
    println!("{} {}", p1.x, p2.x);
}