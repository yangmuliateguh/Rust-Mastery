fn analyze(numbers: &[i32]) -> String {
    match numbers {
        &[] => "empty".into(),
        &[x] => format!("Single:{}", x),
        &[first, .., last] => format!("First: {}, Last: {}", first, last),
    }
}

fn main(){
    println!("{}", analyze(&[]));
    println!("{}", analyze(&[1]));
    println!("{}", analyze(&[1,2,3,4,5]));
}