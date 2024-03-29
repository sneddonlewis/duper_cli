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

#[test]
fn no_duplicates() -> Result<(), Box<dyn Error>>{
    let dir = TempDir::new()?;
    let file = dir.child("sample.txt");
    let file_dup = dir.child("sample_dup.txt");

    file.write_str("hello")?;
    file_dup.write_str("hello, world!")?;

    let mut cmd = Command::cargo_bin("duper_cli")?;

    cmd.arg("--path").arg(dir.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No duplicates found"));

    Ok(())
}

#[test]
fn duplicates_in_nested_directories() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let nested = dir.child("nested");
    let file = dir.child("sample.txt");
    let file_dup = nested.child("sample_dup.txt");

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

#[test]
fn searches_specified_extensions() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let nested = dir.child("nested");
    let file = dir.child("sample.txt");
    let file_dup = nested.child("sample_dup.txt");

    file.write_str("hello")?;
    file_dup.write_str("hello")?;

    let mut cmd = Command::cargo_bin("duper_cli")?;

    cmd.arg("--path").arg(dir.path())
        .arg("--extension").arg(".pdf");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No duplicates found"));

    Ok(())
}
