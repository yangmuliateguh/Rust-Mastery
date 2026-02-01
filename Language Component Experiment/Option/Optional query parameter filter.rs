fn search_user(name: Option<&str>) -> Vec<&str> {
    let all = vec!["ambatukam", "asikjuga", "rusdi"];
    match name {
        Some(q) => all.into_iter().filter(|u| u.contains(q)).collect(),
        None => all,
    }
}

fn main(){
    println!("{:?}", search_user(Some("a")));
    println!("{:?}", search_user(Some("i")));
    println!("{:?}", search_user(Some("m")));
}