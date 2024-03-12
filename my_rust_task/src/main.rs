use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    servers: Vec<Server>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Server {
    socket_address: String,
    response: serde_json::Value,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("../example.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_json::from_str(&contents)?;

    for server in config.servers {
        let socket_address = server.socket_address.clone();
        let response = match server.response {
            serde_json::Value::String(s) => s.into_bytes(),
            serde_json::Value::Array(arr) => {
                arr.iter().map(|v| v.as_u64().unwrap() as u8).collect()
            }
            _ => continue,
        };

        tokio::spawn(async move {
            let listener = TcpListener::bind(&socket_address).await.unwrap();
            println!("Server running on {}", &socket_address);

            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let response = response.clone();
                tokio::spawn(async move {
                    socket.write_all(&response).await.unwrap();
                });
            }
        });
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10000)).await;
    }
}
