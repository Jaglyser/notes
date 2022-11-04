use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub title: Option<String>,
    #[clap(short, long, value_parser)]
    pub note: Option<String>,
    #[clap(short, long, value_parser, takes_value = false)] 
    pub list: bool
}

