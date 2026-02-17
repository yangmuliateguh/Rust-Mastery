use std::any::type_name;

fn type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn box_maybe_value(values: Option<i32>) -> Option<Box<i32>> {
    values.map(Box::new)
}

fn main(){
    let boxed_some = box_maybe_value(Some(20));
    type_of(&boxed_some);
    let boxed_none = box_maybe_value(None);
    type_of(&boxed_none);

    println!("some: {:?}", boxed_some);
    println!("none: {:?}", boxed_none);
}