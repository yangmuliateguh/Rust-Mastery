#[derive (Debug)]
struct Config{
    host: String,
    port: u16
}

fn print_host(c: &Config){
    println!("host: {}", c.host);
}

fn main(){
    let cfg = Config{
        host: "localhost".into(),
        port: 8080
    };
    print_host(&cfg);
    println!("{:?}", cfg);
}