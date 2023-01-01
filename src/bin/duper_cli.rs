use std::fs;
use std::path::PathBuf;
use clap::Parser;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));
    println!("Path: {:?}", path);

    for file in fs::read_dir(path).unwrap() {
        println!("{}", file.unwrap().path().display())
    }
}