use std::rc::Rc;
use std::cell::RefCell;

fn main(){
    let shared = Rc::new(RefCell::new(String::from("hello")));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    clone1.borrow_mut().push_str(" world");
    clone2.borrow_mut().push_str("!");

    println!("{}", shared.borrow());
}