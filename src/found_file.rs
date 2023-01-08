use hex_literal::hex;
use md5::{Digest, Md5};
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::{metadata, read_to_string};
use std::hash::Hash;
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
    files: HashMap<u64, Vec<DuplicateFile>>,
}

pub struct Duplicates {
    files: BTreeMap<String, Vec<DuplicateFile>>,
}

impl Duplicates {
    pub fn from_file_list(file_list: &mut FileList) -> Duplicates {
        let mut result = Duplicates {
            files: BTreeMap::new(),
        };
        for files in file_list.files.values_mut() {
            // any files with the same hash
            // put them into their own list value with key hash
            // sort by hash
            // dedup and put the dups in the result
            files.sort_by(|f, s| f.hash.cmp(&s.hash));

            partition_by_duplicate_hash(files)
                .iter()
                .for_each(|(hash, file_list)| {
                    result.files.insert(hash.to_string(), file_list.to_vec());
                });
        }

        // remove unique hashes
        result.files.retain(|k, v| v.len() > 1);

        result
    }

    pub fn list_files(&self) {
        self.files.iter().for_each(|(key, file_list)| {
            println!("{} hash", key);
            file_list
                .iter()
                .map(|file| file.path.clone())
                .for_each(|path| {
                    println!("\t{:?}", path);
                });
        });
    }
}

fn partition_by_duplicate_hash(files: &Vec<DuplicateFile>) -> BTreeMap<String, Vec<DuplicateFile>> {
    let hashes = files
        .into_iter()
        .map(|f| f.hash.clone())
        .collect::<Vec<String>>();

    let mut result: BTreeMap<String, Vec<DuplicateFile>> = BTreeMap::new();

    files
        .iter()
        .filter(|f| hashes.contains(&f.hash))
        .for_each(|file| {
            match result.contains_key(&file.hash) {
                true => {
                    result
                        .entry(file.hash.clone())
                        .and_modify(|list| list.push(file.clone()));
                }
                false => {
                    result.insert(file.hash.clone(), vec![file.clone()]);
                }
            };
        });
    result.clone()
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
            if entry.path().extension().unwrap() != OsStr::new(&extension_filter.as_ref().unwrap())
            {
                continue;
            }
        }
        let file_metadata = metadata(entry.path());
        if file_metadata.is_err() {
            continue;
        }
        let file_size = file_metadata.unwrap().len();

        match files_by_size.contains_key(&file_size) {
            // push found file into vec
            true => {
                files_by_size.entry(file_size).and_modify(|f| {
                    f.push(FoundFile {
                        path: entry.path().to_owned(),
                        size: file_size,
                    })
                });
            }
            false => {
                files_by_size.insert(
                    file_size,
                    vec![FoundFile {
                        path: entry.path().to_owned(),
                        size: file_size,
                    }],
                );
            }
        };
    }

    // Delete file size entries with only one file as cannot be duplicates
    files_by_size.retain(|_, list| list.len() > 1);
    // Get file hashes
    let mut duplicates: HashMap<u64, Vec<DuplicateFile>> = HashMap::new();
    files_by_size.values().for_each(|file_group| {
        let mut potential_duplicates: Vec<DuplicateFile> = vec![];
        file_group.iter().for_each(|file| {
            // map to dup file with hash
            // read the file and pass it's contents to hasher.update()
            let contents = read_to_string(&file.path);
            // Skip file if unable to read
            if contents.is_err() {
                return;
            }
            let hash = get_md5_hash_from_string(contents.unwrap());
            potential_duplicates.push(DuplicateFile {
                path: file.path.clone(),
                size: file.size,
                hash,
            });
        });

        match potential_duplicates.len() {
            0 => {}
            _ => {
                duplicates.insert(
                    potential_duplicates.first().unwrap().size,
                    potential_duplicates,
                );
            }
        };
    });

    FileList {
        files: duplicates.to_owned(),
    }
}

fn get_md5_hash_from_string(content: String) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(content);
    format!("{:x}", &hasher.finalize())
}
