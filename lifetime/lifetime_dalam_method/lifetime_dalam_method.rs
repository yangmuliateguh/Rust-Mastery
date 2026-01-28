#[derive (Debug)]
struct Article<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Article<'a> {
    fn combined(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }

    fn get_title(&self) -> &'a str {
        self.title
    }
}

fn main(){
    let art = Article {
        title: "Rust Mastery",
        author: "me"
    };
    println!("{:?}", art);
    println!("{}", art.get_title());
}