use clap::Parser;
use duper::duplicate_files::{DuplicateFiles};
use std::path::PathBuf;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;
    println!("Extension Filter: {:?}", extension_filter);

    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    let file_list = DuplicateFiles::new(path, extension_filter);
    file_list.list_files();
}
