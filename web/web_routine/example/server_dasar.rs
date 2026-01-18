use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box <dyn std::error::Error>>{
    // 1. Setup alamat dan listener
    let address = SocketAddr::from(([127,0,0,1], 3000));
    let listener = TcpListener::bind(&address).await?;
    println!("menunggu koneksi di {}", address);

    // 2. Terima koneksi terus menerus
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("koneksi diterima dari {}", addr);

        // 3. Tutup koneksi ketika sudah connect
        drop(socket);
    }
}