fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main(){
    let result;
    {
        let s1 = String::from("coding");
        let s2 = String::from("asikjuga");
        result = longest(&s1, &s2);
        println!("{}", result);
    }
}