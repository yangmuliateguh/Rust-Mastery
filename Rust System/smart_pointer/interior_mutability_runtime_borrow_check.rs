use std::cell::RefCell;

fn main(){
    let data = RefCell::new(5);
    println!("{}", data.borrow());
    *data.borrow_mut() += 10;
    println!("{}", data.borrow());

    let r1 = data.borrow();
    let r2 = data.borrow_mut(); //panic
}