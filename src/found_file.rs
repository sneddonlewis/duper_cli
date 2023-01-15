use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FoundFile {
    pub(crate) path: PathBuf,
    pub(crate) size: u64,
}

impl FoundFile {
    pub fn new(path: PathBuf, size: u64) -> Self {
        FoundFile {
            path,
            size,
        }
    }
}