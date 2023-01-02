use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{metadata, read_to_string};
use md5::{Md5, Digest};
use hex_literal::hex;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct FoundFile {
    path: PathBuf,
    size: u64,
}

#[derive(Debug, Clone)]
struct DuplicateFile {
    path: PathBuf,
    size: u64,
    hash: String,
}

#[derive(Debug)]
pub struct FileList {
    files: HashMap<u64, Vec<FoundFile>>,
}

impl FileList {
    pub fn list_files(&self) {
        for (key, file_list) in self.files.iter() {
            println!("{:?} bytes", key);
            for file in file_list {
                println!("\t{:?}", file.path);
            }
        }
    }
}

pub fn new_file_list(base_path: PathBuf, extension_filter: Option<String>) -> FileList {
    let mut files_by_size: HashMap<u64, Vec<FoundFile>> = HashMap::new();

    for entry in WalkDir::new(base_path.as_path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir())
    {
        if extension_filter.is_some() {
            // skip files without extension if there is a filter
            if entry.path().extension().is_none() {
                continue;
            }
            if entry.path().extension().unwrap() != OsStr::new(&extension_filter.as_ref().unwrap()) {
                continue;
            }
        }
        let file_size = metadata(entry.path()).unwrap().len();

        if files_by_size.contains_key(&file_size) {
            // push found file into vec
            files_by_size.entry(file_size).and_modify(|f| {
                f.push(FoundFile {
                    path: entry.path().to_owned(),
                    size: file_size,
                })
            });
        } else {
            // create a new vec with the file if size not seen
            files_by_size.insert(file_size, vec![FoundFile {
                path: entry.path().to_owned(),
                    size: file_size,
            }]);
        }
    }

    // Delete file size entries with only one file as cannot be duplicates
    files_by_size
        .retain(|_, list| list.len() > 1);
    // Get file hashes
    let mut duplicates: HashMap<u64, Vec<DuplicateFile>> = HashMap::new();
    for file_group in files_by_size.values() {
        let mut potential_duplicates: Vec<DuplicateFile> = vec![];
        for file in file_group {
            // map to dup file with hash
            let mut hasher = md5::Md5::new();
            // read the file and pass it's contents to hasher.update()
            let contents = read_to_string(&file.path).unwrap();
            hasher.update(contents);
            let hash = format!("{:x}", &hasher.finalize());
            potential_duplicates.push(DuplicateFile {
                path: file.path.clone(),
                size: file.size,
                hash,
            })
        }
        duplicates.insert(potential_duplicates.first().unwrap().size, potential_duplicates);
    }
    println!("{:?}", duplicates);
    // Collect files into lists mapped by file hash (list of duplicates)

    FileList {
        files: files_by_size.to_owned(),
    }
}
