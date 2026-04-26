use std::hash::Hash;
use sha2::digest::typenum::Add1;
use tokio::{
    net::{TcpListener, TcpStream}
};
use tokio::io::AsyncReadExt;
use crate::core::blockchain::Blockchain;
use crate::network::message::Message;
use crate::network::peer::broadcast_message;

pub async fn start_connection(addr: &str, blockchain: &Blockchain) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on port: {}", addr);

    loop{
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handler(&mut socket, &addr.to_string(), blockchain.clone()).await;
        });
    }
}

pub async fn handler(stream: &mut TcpStream, addr: &str, blockchain: &Blockchain) {
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();

    let message: Message = serde_json::from_slice(&buffer[..n]).unwrap();

    match message {
        Message::NewTransaction(tx) => {
            print!("New transaction");
            todo!()
        },
        Message::NewBlock(block) => {
            print!("New block");
            todo!()
        },
        Message::RequestChain => {
            broadcast_message(addr, &Message::ResponseChain(blockchain.chain.clone())).await;
        },
        Message::ResponseChain(chain) => {
            
        }
    }
}