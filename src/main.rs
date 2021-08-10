// ITER 3

use tokio::{
    io::{
        AsyncBufReadExt, 
        AsyncWriteExt, 
        BufReader
    },
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("{}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 { break; }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

// ITER 2

// use tokio::{
//     io::{
//         AsyncBufReadExt, 
//         AsyncWriteExt, 
//         BufReader
//     },
//     net::TcpListener,
// };

// #[tokio::main]
// async fn main() {
//     let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();

//     loop {
//         let (mut socket, addr) = listener.accept().await.unwrap();
//         println!("{}", addr);

//         tokio::spawn(async move {
//             let (reader, mut writer) = socket.split();

//             let mut reader = BufReader::new(reader);
//             let mut line = String::new();

//             loop {
//                 let bytes_read: usize = reader.read_line(&mut line).await.unwrap();
//                 if bytes_read == 0 { break; }

//                 writer.write_all(&line.as_bytes()).await.unwrap();
//                 line.clear();
//             }
//         });
//     }
// }

// ITER 1

// use tokio::{
//     io::{AsyncReadExt, AsyncWriteExt},
//     net::TcpListener,
// };

// #[tokio::main]
// async fn main() {
//     let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();

//     let (mut socket, addr) = listener.accept().await.unwrap();

//     println!("{}", addr);

//     loop {
//         let mut buffer = [0u8; 1024];

//         let bytes_read: usize = socket.read(&mut buffer).await.unwrap();
    
//         socket.write_all(&buffer[..bytes_read]).await.unwrap();    
//     }
// }
