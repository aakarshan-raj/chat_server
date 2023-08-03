use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("localhost:8080").await.unwrap();
    loop{
    let (mut socket, _addr) = listner.accept().await.unwrap();

    tokio::spawn(async move {

            let (read, mut write) = socket.split();

            let mut buffer = BufReader::new(read);
            loop {
                let mut lines = String::new();
                write.write(&mut lines.as_bytes()).await.unwrap();
                print!("Message>{}", lines);
            }
    });
}
}
