struct Person {
    name: String,
    age: u8
}

fn main(){
    let p1 = Person {
        name: "joko".into(),
        age: 20
    };
    let p2 = p1;
    // println!("{}", p1.age);
    println!("{}", p2.age);
}