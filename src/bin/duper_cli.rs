use clap::Parser;
use duper::duplicate_files::{DuplicateFiles};
use std::path::PathBuf;
use colored::{Colorize, ColoredString};

fn main() {
    println!("{}", get_title());
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;
    if extension_filter.is_some() {
        println!("Extension Filter: {:?}", extension_filter);
    }

    let path = opts.path.unwrap_or(PathBuf::from(r"./test_data"));

    let file_list = DuplicateFiles::search(path, extension_filter);
    file_list.list_files();
}

fn get_title() -> ColoredString {
    let result = "  ____                        \n".to_owned() +
        " |  _ \\ _   _ _ __   ___ _ __ \n" +
        " | | | | | | | '_ \\ / _ \\ '__|\n" +
        " | |_| | |_| | |_) |  __/ |   \n" +
        " |____/ \\__,_| .__/ \\___|_|   \n" +
        "             |_|              \n\n";
    result.green()
}
