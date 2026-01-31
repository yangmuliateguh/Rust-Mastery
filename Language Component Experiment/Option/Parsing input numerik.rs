fn main(){
    let parse_age = |input: &str| -> Option<u8> {
        input.parse().ok()
    };

    for input in ["25", "invalid", "20"] {
        let age = parse_age(input);
        println!("{} : {}", input, age.is_some());
    }
}