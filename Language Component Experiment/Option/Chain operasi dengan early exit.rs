fn main(){
    let config: Option<i32> = Some(123);
    if config.is_some() {
        println!("{}", config.unwrap());
    } else {
        println!("none");
    }
}