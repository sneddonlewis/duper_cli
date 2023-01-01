use clap::Parser;
use duper::found_file::new_file_list;
use std::path::PathBuf;

fn main() {
    println!("Duper");
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;
    println!("Extension Filter: {:?}", extension_filter);

    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    let file_list = new_file_list(path, extension_filter);
    file_list.list_files();
}
