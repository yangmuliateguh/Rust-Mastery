fn main(){
    let users = vec!["ambatukam", "rusdi", "kakangku"];
    let result: Vec<&str> = users.into_iter()
        .filter(|u| u.contains("a"))
        .collect();
    println!("{:?}", result);
}