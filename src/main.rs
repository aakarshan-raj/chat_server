
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() {

    let listner = TcpListener::bind("localhost:8030").await.unwrap();
    loop{
    let (mut socket,_addr) = listner.accept().await.unwrap();
   
    let ( read,mut write) = socket.split();

    let mut buffer = BufReader::new(read);
    let mut lines = String::new();

    let size = buffer.read_line(&mut lines).await.unwrap();
    write.write(&mut lines.as_bytes()).await.unwrap();

    print!("Message>{}",lines);
    }



}