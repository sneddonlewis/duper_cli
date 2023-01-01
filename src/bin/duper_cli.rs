use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));
    println!("Path: {:?}", path);

    for entry in WalkDir::new(path.as_path()).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}