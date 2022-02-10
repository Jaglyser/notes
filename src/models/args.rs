use Clap::parser;

#[derive(Parser)]
struct Cli {
    time: String,
    delete: Id,
    #[clap(parse(from_os_str))]
    path: std::path::Pathbuff,
}
fn main() {
    let args = Cli::parse();
}
