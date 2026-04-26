use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::network::message::Message;

pub struct Network {
    peers: Vec<String>,
}

impl Network {
    pub async fn broadcast(&self, msg: &Message) {
        for peer in &self.peers {
            broadcast_message(&peer[..], msg).await;
        }
    }
}

pub async fn broadcast_message(addr: &str, msg: &Message) {
    let mut stream = TcpStream::connect(addr).await.unwrap();
    let data = serde_json::to_vec(msg).unwrap();
    let _ = stream.write_all(&data[..]).await.unwrap();
}