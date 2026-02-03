#[derive (Debug)]
struct Config {
    host: String,
    port: u16
}

fn print_host(c: &Config) {
    println!("Host : ", c.host);
}

fn main (){
    let cfg = Config {
        host: "localhost".into(),
        port: 8000,
    };
    print_host(&cfg);
    println!("{:?}", cfg);
}