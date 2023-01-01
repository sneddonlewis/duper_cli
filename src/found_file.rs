use std::path::PathBuf;
use walkdir::WalkDir;

struct FoundFile {
    path: PathBuf,
}

pub struct FileList {
    files: Vec<FoundFile>,
}

impl FileList {
    pub fn list_files(&self) {
        // for file in self.files.into_iter() {
        //     println!("{:?}", file.path);
        // }
        println!("list files here");
    }
}

pub fn new_file_list(base_path: PathBuf, extension_filter: Option<String>) -> FileList {
    let mut files: Vec<FoundFile> = Vec::new();

    for entry in WalkDir::new(base_path.as_path()).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        if extension_filter.is_some() {
            // skip files without extension if there is a filter
            if entry.path().extension().is_none() {
                continue;
            }
        }
        files.push(FoundFile { path: entry.path().to_owned() })
    }

    FileList {
        files
    }
}