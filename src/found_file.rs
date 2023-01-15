use md5::Digest;
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::{metadata, read_to_string};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FoundFile {
    pub(crate) path: PathBuf,
    pub(crate) size: u64,
}

#[derive(Debug, Clone)]
struct DuplicateFile {
    path: PathBuf,
    size: u64,
    hash: String,
}