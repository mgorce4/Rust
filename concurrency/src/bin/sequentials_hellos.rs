use anyhow::Ok;
use clap::Parser;

#[derive(Debug, Parser)]
struct Parameters {
    n: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let param = Parameters::parse();
    let mut my_tasks = vec![];



    for i in 0..param.n {
        my_tasks.push(tokio::spawn(async move {
            println!("Bonjour {}", i);
            println!("Au revoir {}", i);
        }));
    }

    for task in my_tasks {
        task.await.expect("Task panicked");
    }
    Ok(())
}