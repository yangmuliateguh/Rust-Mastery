fn main(){
    match "3.14".parse::<f64>(){
        Ok(v) => println!("{}",v),
        Err(e) => println!("{}",e),
    }
}