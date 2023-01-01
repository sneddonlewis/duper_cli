use std::path::PathBuf;
use clap::Parser;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    // replace with pwd
    let path = opts.path.unwrap_or(PathBuf::from(r"~/CLionProjects/duper/test_data"));
    println!("Path: {:?}", path);
}