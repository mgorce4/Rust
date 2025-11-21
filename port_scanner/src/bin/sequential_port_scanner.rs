use port_scanner::{is_open, Parameters};
use clap::Parser;

#[tokio::main]
async fn main() {
    let my_parameters = Parameters::parse();
    
    use std::time::Instant;
    let instant = Instant::now();

    println!("Scanning {}:{}-{}", 
        my_parameters.host, 
        my_parameters.port_min, 
        my_parameters.port_max
    );
    
    for port in my_parameters.port_min..=my_parameters.port_max {
        if is_open(&my_parameters.host, port, my_parameters.timeout).await {
            println!("Port {} is open", port);
        }
    }
    
    println!("Scan complete");
    println!("{:?}", instant.elapsed());
}