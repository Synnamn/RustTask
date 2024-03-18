use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct Server {
    socket_address: String,
    response: Vec<ResponseEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseEntry {
    index: usize,
    value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Value {
    Integer(u64),
    Char(char),
}

fn main() -> io::Result<()> {
    // Read input from file
    let input_json = fs::read_to_string("../example.json")?;
    let parsed_json: serde_json::Value = serde_json::from_str(&input_json)?;

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
                serde_json::Value::String(s) => {
                    s.chars()
                        .enumerate()
                        .map(|(i, c)| ResponseEntry {
                            index: i,
                            value: Value::Char(c),
                        })
                        .collect()
                }
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .enumerate()
                        .filter_map(|(i, v)| v.as_u64().map(|num| {
                            let ch = (num as u8) as char;
                            ResponseEntry {
                                index: i,
                                value: Value::Char(ch),
                            }
                        }))
                        .collect()
                }
                _ => Vec::new(),
            };
            Server {
                socket_address,
                response,
            }
        })
        .collect();

    // Create output JSON
    let mut output = String::new();
    output.push_str("{\n");
    output.push_str("  \"servers\": [\n");
    for (i, server) in servers.iter().enumerate() {
        output.push_str("    {\n");
        output.push_str(&format!("      \"socket_address\": \"{}\",\n", server.socket_address));
        output.push_str("      \"response\": {\n");
        for entry in &server.response {
            match &entry.value {
                Value::Char(ch) => {
                    if entry.index < &server.response.len() - 1 {
                    output.push_str(&format!("        \"{}\": \"{}\",\n", entry.index, ch));}
                    else {
                        output.push_str(&format!("        \"{}\": \"{}\"\n", entry.index, ch));
                    }
                }
                _ => {}
            }
        }
        output.push_str("      }\n");
        if i < servers.len() - 1 {
            output.push_str("    },\n");
        } else {
            output.push_str("    }\n");
        }
    }
    output.push_str("  ]\n");
    output.push_str("}\n");

    // Write output JSON to file
    fs::write("../output.json", output)?;

    Ok(())
}
