use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    // does not follow sym links
    // walks hidden files
    for entry in WalkDir::new(path.as_path()).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        println!("{}", entry.path().display());
    }
}