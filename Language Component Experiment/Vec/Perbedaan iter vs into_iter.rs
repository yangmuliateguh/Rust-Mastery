fn main(){
    let users = vec!["joko", "giorno", "diana", "ambatukam"];
    {
        let refs: Vec<&&str> = users.iter()
            .filter(|n| (*n).contains("jo"))
            .collect();
        println!("{:?}", refs);
    }
    
    let refs2: Vec<&str> = users.into_iter()
        .filter(|n| n.contains("i"))
        .collect();
    println!("{:?}", refs2);
}