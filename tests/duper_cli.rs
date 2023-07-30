use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;
use assert_fs::TempDir;

#[test]
fn duplicates_in_same_directory() -> Result<(), Box<dyn Error>>{
    let dir = TempDir::new()?;
    let file = dir.child("sample.txt");
    let file_dup = dir.child("sample_dup.txt");

    file.write_str("hello")?;
    file_dup.write_str("hello")?;

    let mut cmd = Command::cargo_bin("duper_cli")?;

    cmd.arg("--path").arg(dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sample.txt"))
        .stdout(predicate::str::contains("sample_dup.txt"));

    Ok(())
}
