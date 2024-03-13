use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    socket_address: String,
    response: Value,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input from file
    let input_json = fs::read_to_string("../example.json")?;
    let parsed_json: Value = serde_json::from_str(&input_json)?;

    // Process input JSON
    let servers: Vec<Server> = parsed_json["servers"]
        .as_array()
        .ok_or("Invalid input JSON")?
        .iter()
        .map(|server| {
            let socket_address = server["socket_address"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            let response = match &server["response"] {
                Value::String(s) => s.chars().map(|c| c.to_string()).collect(),
                Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_u64().map(|c| c as u8 as char).map(|c| c.to_string()))
                    .collect(),
                _ => Vec::new(),
            };
            Server {
                socket_address,
                response: json!(response),
            }
        })
        .collect();

    // Create output JSON
    let output_json = json!({
        "servers": servers.iter().map(|server| {
            json!({
                "socket_address": &server.socket_address,
                "response": &server.response
            })
        }).collect::<Vec<_>>()
    });

    // Print output to console
    println!("{}", serde_json::to_string_pretty(&output_json)?);

    Ok(())
}
