use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("192.168.1.5:8080").await.unwrap();
    let (tx, rx) = broadcast::channel(10);
    loop {
        let txx = tx.clone();
        let mut rxx = txx.subscribe();
        let (mut socket, addr) = listner.accept().await.unwrap();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut buffer = BufReader::new(read);
            loop {
                let mut lines = String::new();
                tokio::select!{
                result = buffer.read_line(&mut lines)=>{
                if result.unwrap() == 2{
                    break;
                }    
                txx.send((lines.clone(),addr)).unwrap();
                
                }
               
                result = rxx.recv() =>{
                let (msg,addr_check) = result.unwrap();    
                if addr != addr_check{
                write.write(&mut msg.as_bytes()).await.unwrap();
                }
                }
                }
            }
        });
    }
}
