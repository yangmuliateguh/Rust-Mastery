use std::fmt::Display;

trait Log {
    fn log(&self);
}

struct Event {
    msg: String
}

impl Display for Event {
    fn fmt(
        &self, f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Log for Event {
    fn log(&self) {
        println!("[LOG]: {}", self);
    }
}

fn print_log<T: Display + Log>(item: T) {
    println!("[System]: {}", item);
    item.log();
}

fn main(){
    let e = Event{ msg:"asikjuga cuyy".into() };
    print_log(e);
}