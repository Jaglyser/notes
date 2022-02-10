use clap::Parser;
#[derive(Parser)]
struct Cli {
    time: String,
    message: String,
}
fn main() {
    let _args = Cli::parse();
}
