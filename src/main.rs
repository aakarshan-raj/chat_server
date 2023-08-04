use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, rx) = broadcast::channel::<String>(10);
    loop {
        let txx = tx.clone();
        let mut rxx = txx.subscribe();
        let (mut socket, _addr) = listner.accept().await.unwrap();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut buffer = BufReader::new(read);
            loop {
                let mut lines = String::new();
                tokio::select!{
                result = buffer.read_line(&mut lines)=>{
                txx.send(lines.clone()).unwrap();
                }
               
                result = rxx.recv() =>{
                let msg = result.unwrap();    
                write.write(&mut msg.as_bytes()).await.unwrap();
                print!("Message>{}", lines);
                }
                }
            }
        });
    }
}
