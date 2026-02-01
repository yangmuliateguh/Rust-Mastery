use serde::Deserialize;
use serde_json;

#[derive (Debug,Deserialize)]
struct User {
    email: Option<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let raw = r#"{"email": null}"#;
    let user: User = serde_json::from_str(raw).unwrap();
    println!("{:?}", user);
    println!("{}", user.email.is_some());
    Ok(())
}