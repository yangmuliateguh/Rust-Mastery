use std::sync::mpsc;
use std::thread;

fn kirim_data(tx: mpsc::Sender<i32>, data: i32) {
    tx.send(data).unwrap();
}

fn main(){
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        kirim_data(tx, 21)
    });

    let received = rx.recv().unwrap();
    println!("data diterima: {}", received);
    handle.join().unwrap();
}