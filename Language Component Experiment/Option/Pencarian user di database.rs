fn main(){
    let users = vec!["joko", "jono", "ambatukam"];
    let find_user = |name: &str| -> Option<&str> {
        users.iter().find(|&&u| u == name).copied()
    };

    let result = find_user("asik");
    println!("some? {}", result.is_some());
    println!("none? {}", result.is_none());
}