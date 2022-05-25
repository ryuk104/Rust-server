use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind(addr: "localhost:8080").await.unwrap();
    loop{
        let(mut socket:TcpStream,_addr:SocketAddr)=listener.accept().await.unwrap();
        let(reader:ReadHalf,mut writer:WriteHalf)=socket.split();
        let mut reader:BufReader<ReadHalf>=BufReader :: new(inner:reader);
        let mut line:string=String :: new();
        loop{
            let bytes_read:usize=reader.read_line(buf:&mut line).await.unwrap();
            if bytes_read ==0{
                break;
            }
            writer.write_all(src:line.as_bytes()).await.unwrap();
            line.clear();
        }
    }
}