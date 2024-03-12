use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::io::{self, Read};

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    socket_address: String,
    response: Value,
}

fn main() -> io::Result<()> {
    // Read input from file
    let input_json = fs::read_to_string("../example.json")?;
    let parsed_json: Value = serde_json::from_str(&input_json)?;

    // Process input JSON
    let servers: Vec<Server> = parsed_json["servers"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid input JSON"))?
        .iter()
        .map(|server| {
            let socket_address = server["socket_address"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            let response = server["response"].clone();
            let response = match response {
                Value::String(s) => {
                    let mut obj = serde_json::Map::new();
                    for (i, c) in s.chars().enumerate() {
                        obj.insert(i.to_string(), json!(c.to_string()));
                    }
                    Value::Object(obj)
                }
                Value::Array(arr) => {
                    let mut obj = serde_json::Map::new();
                    for (i, byte) in arr.iter().enumerate() {
                        if let Some(c) = byte.as_u64() {
                            obj.insert(i.to_string(), json!(c as u8 as char));
                        }
                    }
                    Value::Object(obj)
                }
                _ => Value::Null,
            };
            Server {
                socket_address,
                response,
            }
        })
        .collect();

    // Create output JSON
    let output_json: Value = json!({
        "servers": servers.iter().map(|server| {
            json!({
                "socket_address": server.socket_address,
                "response": server.response
            })
        }).collect::<Vec<Value>>()
    });

    // Print output to console
    println!("{}", serde_json::to_string_pretty(&output_json)?);

    Ok(())
}
