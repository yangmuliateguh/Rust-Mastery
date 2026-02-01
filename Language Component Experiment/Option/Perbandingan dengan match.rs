fn main(){
    let email: Option<String> = None;
    match email {
        Some(_) => println!("some"),
        None => println!("none")
    }
}