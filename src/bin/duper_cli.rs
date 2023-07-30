use clap::{App, Parser};
use duper::duplicate_files::{DuplicateFiles};

fn main() {
    let mut app = App::new("Duper")
        .author("Lewis Sneddon")
        .version("0.1.0");
    let opts = duper::opts::Opts::parse();
    let extension_filter = opts.extension;

    match opts.path {
        Some(path) => {
            let file_list = DuplicateFiles::search(path, extension_filter);
            if file_list.has_duplicates() {
                file_list.list_files();
            } else {
                panic!("No duplicates found in directory");
            }
        },
        None => {
            app.print_help().unwrap();
        }
    };
}
