fn main(){
    let mut s = String::from("coba ini deh");
    
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    
    println!("{} {}", r1,r2);
    let r3 = &mut s;
    r3.push_str("!");
    println!("{}", r3);
}