use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Siapkan dan bind server TCP ke alamat lokal
    let address = SocketAddr::from(([127,0,0,1], 3000));
    let listener = TcpListener::bind(&address).await?;
    println!("📡 Menunggu koneksi...");

    // 2. Terima koneksi masuk secara terus-menerus
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("✅ Koneksi dari: {}", addr);

        // 3. Baca data mentah dari klien
        let mut buffer = [0u8; 256];
        if let Ok(n) = socket.read(&mut buffer).await {
            println!("📦 Diterima {} byte:", n);
            for b in &buffer[..n] {
                print!("{:02x} ", b);
            }
            println!();
        }

        // 4. Tutup koneksi setelah membaca
        drop(socket);
    }
}