fn create_boxed_value(value: i32) -> Box<i32> {
    Box::new(value)
}

fn main(){
    let boxed = create_boxed_value(20);
    println!("{}", boxed);
    println!("{:p}",boxed);
}