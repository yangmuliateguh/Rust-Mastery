trait Shape{
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn create_shape() -> impl Shape {
    Rectangle { width: 20.0, height: 30.0 }
}

fn main(){
    let s = create_shape();
    println!("{}", s.area());

}