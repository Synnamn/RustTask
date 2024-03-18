use std::env;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <address>");
        return Ok(());
    }

    let mut stream = TcpStream::connect(&args[1]).await?;
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let parsed_response = buffer
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            if b.is_ascii_digit() {
                format!("\n\"{}\": {}", i, b as char)
            } else {
                format!("\n\"{}\": \"{}\"", i, b as char)
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    println!("{{ {} \n}}", parsed_response);

    Ok(())
}
