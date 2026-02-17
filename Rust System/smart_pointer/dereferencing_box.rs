use std::any::type_name;

fn type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn unwrap_boxed_value(boxed: Box<i32>) -> i32 {
    *boxed
}

fn main(){
    let boxed = Box::new(200);
    type_of(&boxed);
    let value = unwrap_boxed_value(boxed);
    type_of(&value);
    println!("{}", value);
}