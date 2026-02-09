#[derive(clap::Parser, Debug)]

pub struct Configuration {
    #[clap(short, long, required = true, num_args = 1..)]
    pub candidates: Vec<String>,
}
