use std::fs::read;
use md5::Digest;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FoundFile {
    path: PathBuf,
    size: u64,
}

impl FoundFile {
    pub fn new(path: PathBuf, size: u64) -> Self {
        FoundFile {
            path,
            size,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn md5_hash(&self) -> Result<String, &str> {
        let binary_contents = read(&self.path);
        if binary_contents.is_err() {
            return Err("Unable to read file");
        }
        let hash = get_md5_hash_from_contents(binary_contents.unwrap());
        Ok(hash)
    }
}

fn get_md5_hash_from_contents(content: Vec<u8>) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(content);
    format!("{:x}", &hasher.finalize())
}
