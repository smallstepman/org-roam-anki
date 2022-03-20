use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub path: String,
    pub ignore_tags: Vec<String>,
    pub ignore_directories: Vec<String>,
}

impl Args {
    pub fn get_args() -> Self {
        Args::parse()
    }
}
