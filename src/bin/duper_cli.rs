use clap::Parser;
use duper::found_file::{new_file_list, Duplicates};
use std::path::PathBuf;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;
    println!("Extension Filter: {:?}", extension_filter);

    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    let mut file_list = new_file_list(path, extension_filter);
    let duplicates = Duplicates::from_file_list(&mut file_list);
    duplicates.list_files();
}
