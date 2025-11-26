use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    port: u16,
}

const LOCALHOST: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parameters = Parameters::parse();
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, parameters.port)).await?;
    println!("TCP echo server listening on {}:{}", LOCALHOST, parameters.port);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        
        // Créer une tâche pour chaque client
        tokio::spawn(async move {
            let (reader, mut writer) = stream.into_split();
            let mut lines = BufReader::new(reader).lines();
            
            // Lire et renvoyer chaque ligne reçue
            while let Ok(Some(line)) = lines.next_line().await {
                println!("Received from {}: {}", addr, line);
                
                // Renvoyer exactement la même ligne (echo) avec \n
                let response = format!("{}\n", line);
                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    eprintln!("Error writing to {}: {}", addr, e);
                    break;
                }
                
                if let Err(e) = writer.flush().await {
                    eprintln!("Error flushing to {}: {}", addr, e);
                    break;
                }
            }
            
            println!("Connection closed: {}", addr);
            anyhow::Ok(())
        });
    }
}
