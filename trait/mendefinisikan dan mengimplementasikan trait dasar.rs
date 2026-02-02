trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("hello {}", self.name)
    }
}

fn main(){
    let p = Person {name:"joko".into()};
    println!("{}", p.greet());
}