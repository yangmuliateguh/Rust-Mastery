fn main (){
    let data = create();
    let data = transform(data);
    display(data);
}

fn create() -> String {
    String::from("awal")
}

fn transform(s: String) -> String {
    format!("{} -> akhir", s)
}

fn display(s: String) {
    println!("{}",s);
}