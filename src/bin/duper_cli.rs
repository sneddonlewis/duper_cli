use clap::Parser;
use duper::duplicate_files::{DuplicateFiles};
use std::path::PathBuf;

fn main() {
    println!("  ____                        ");
    println!(" |  _ \\ _   _ _ __   ___ _ __ ");
    println!(" | | | | | | | '_ \\ / _ \\ '__|");
    println!(" | |_| | |_| | |_) |  __/ |   ");
    println!(" |____/ \\__,_| .__/ \\___|_|   ");
    println!("             |_|              ");
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;
    if extension_filter.is_some() {
        println!("Extension Filter: {:?}", extension_filter);
    }

    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    let file_list = DuplicateFiles::search(path, extension_filter);
    file_list.list_files();
}
