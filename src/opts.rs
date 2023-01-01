use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    pub args: Vec<String>,

    #[clap(short = 'p', long = "path")]
    pub path: Option<PathBuf>,

    #[clap(short = 'e', long = "extension")]
    pub extension: Option<String>
}