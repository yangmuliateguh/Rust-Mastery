#[derive (Debug)]
struct Highlight<'a> {
    text: &'a str
}

fn main(){
    let content = "penting";
    let h = Highlight{ text: content };
    println!("{:?}", h);
}