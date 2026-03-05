use std::rc::Rc;

fn main() {
    let a = Rc::new(vec![1,2,3]);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("count a: {}", Rc::strong_count(&a));
    drop(b);
    println!("count a after drop b: {}", Rc::strong_count(&a));
    println!("a: {:?}\nc : {:?}", a, c);
}