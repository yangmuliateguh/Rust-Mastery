#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main(){
    let list = List::Cons(12, Box::new(List::Cons(21, Box::new(List::Nil))));
    println!("{:?}", list);
}