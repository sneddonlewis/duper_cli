use std::collections::HashMap;
use std::fs::metadata;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct FoundFile {
    path: PathBuf,
    size: u64,
}

#[derive(Debug)]
pub struct FileList {
    files: HashMap<u64, Vec<FoundFile>>,
}

impl FileList {
    pub fn list_files(&self) {
        for file_list in self.files.keys().into_iter() {
            println!("{:?} bytes", file_list);
        }
    }
}

pub fn new_file_list(base_path: PathBuf, extension_filter: Option<String>) -> FileList {
    let mut files: Vec<FoundFile> = Vec::new();
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
        }
        let file_size = metadata(entry.path()).unwrap().len();
        files.push(FoundFile {
            path: entry.path().to_owned(),
            size: file_size,
        });
        if files_by_size.contains_key(&file_size) {
            // push found file into vec
            files_by_size.entry(file_size).and_modify(|f| f.push(FoundFile{
                path: entry.path().to_owned(),
                size: file_size,
            }));
        } else {
            files_by_size.insert(file_size, vec![FoundFile{
                path: entry.path().to_owned(),
                size: file_size,
            }]);
        }
    }

    FileList {
        files: files_by_size.to_owned(),
    }
}
