use clap::Parser;
use duper::duplicate_files::{DuplicateFiles};

fn main() {
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;

    let path = opts.path.unwrap();

    let file_list = DuplicateFiles::search(path, extension_filter);
    if file_list.has_duplicates() {
        file_list.list_files();
    } else {
        panic!("No duplicates found in directory");
    }
}
