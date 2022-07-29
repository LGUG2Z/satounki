use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub config: PathBuf,
}
