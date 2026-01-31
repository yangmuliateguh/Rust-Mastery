fn main(){
    let invalid: Result<i32, _> = "abc".parse();
    println!("{:?}", invalid)
}