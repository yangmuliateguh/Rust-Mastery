use std::thread;
use std::time::Duration;

fn fetch_url(url: &str) -> String {
    println!("mendapatkan data dari: {}", url);
    thread::sleep(Duration::from_millis(500));
    format!("data dari: {}\nsepanjang: {} bytes", url, url.len())
}

fn main(){
    let urls = vec![
        "https://api.contoh.com/users",
        "https://api.contoh.com/posts",
        "https://api.contoh.com/example",
        "https://api.contoh.com/albums",
    ];

    let mut handles = vec![];

    for url in urls {
        let handle = thread::spawn(move || {
            fetch_url(url)
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("{}", result);
    }
}