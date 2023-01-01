use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;
    println!("Extension Filter: {:?}", extension_filter);

    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    // does not follow sym links
    // walks hidden files
    for entry in WalkDir::new(path.as_path()).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        if extension_filter.is_some() {
            // skip files without extension if there is a filter
            if entry.path().extension().is_none() {
                continue;
            }
        }
        println!("{}", entry.path().display());
    }
}