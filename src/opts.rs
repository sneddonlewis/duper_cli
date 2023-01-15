use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    // pub args: Vec<String>,
    #[clap(
        short = 'p',
        long = "path",
        help = "Path to the directory that will be recursively searched for duplicates. Defaults to the Present Working Directory"
    )]
    pub path: Option<PathBuf>,

    #[clap(
        short = 'e',
        long = "extension",
        help = "File types to search for (by file extension). Defaults to any file type"
    )]
    pub extension: Option<String>,
}
