fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main(){
    let text = "hello world";
    let word = first_word(&text);
    println!("{}", word);
}