fn longest_word(s: &str) -> &str {
    s.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

fn main(){
    let text = "belajar ngoding itu asikjuga tapi beda cerita klo bahasanya rust";
    let word = longest_word(text);
    println!("kata terpanjang {} ", word);
}