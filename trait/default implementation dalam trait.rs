trait Drawable {
    fn draw(&self);
    fn render(&self){
        println!("rendering");
        self.draw();
    }
}

struct Circle;

impl Drawable for Circle {
    fn draw(&self){
        println!("draw a circle");
    }
}

fn main(){
    let c = Circle;
    c.render();
}