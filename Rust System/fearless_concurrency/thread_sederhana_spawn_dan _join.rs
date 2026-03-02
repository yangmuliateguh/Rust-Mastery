use std::thread;

fn print_message(msg: &str){
    println!("{}", msg);
}

fn main(){
    let handle = thread::spawn(|| {
        print_message("ini thread anak");
    });

    print_message("ini thread utama");
    handle.join().unwrap();
}