use std::thread;
use std::fs;

fn process_file(filename: &str) -> Result<usize, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    let word_count = content.split_whitespace().count();
    println!("file: {}\nwords: {}", filename, word_count);
    Ok(word_count)
}

fn main(){
    let files = vec![
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.txt",
    ];

    let mut handles = vec![];

    for file in files {
        let handle = thread::spawn(move || {
            process_file(file)
        });
        handles.push(handle);
    }

    let mut total_words = 0;
    for handle in handles {
        match handle.join().unwrap() {
            Ok(count) => total_words += count,
            Err(e) => eprintln!("Error: {}", e)
        }
    }

    println!("{}", total_words);
}

