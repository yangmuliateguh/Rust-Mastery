trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

fn notify<T: Summary>(item: T) {
    println!("Breaking: {}", item.summarize());
}

fn main(){
    let art = Article { title: "joko asikjuga".into() };
    notify(art);
}