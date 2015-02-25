#![feature(old_io, old_path)]

// Generate the git version string.

use std::old_io::process::{Command, ProcessOutput};
use std::old_io::{File};

fn git_version() -> String {
    match Command::new("git")
        .arg("log").arg("--pretty=format:%h").arg("-1").output() {
        Ok(ProcessOutput { status: exit, output: out, error: err }) => {
            if exit.success() {
                return String::from_utf8(out).unwrap();
            } else {
                println!("Error getting git version: {}", String::from_utf8(err).unwrap());
            }
        }
        Err(err) => {
            println!("Error getting git version: {}", err);
        }
    }
    return "unknown".to_string();
}

fn rustc_version() -> String {
    match Command::new("rustc")
        .arg("--version").output() {
        Ok(ProcessOutput { status: exit, output: out, error: err }) => {
            if exit.success() {
                return String::from_utf8(out).unwrap();
            } else {
                println!("Error getting rustc version: {}", String::from_utf8(err).unwrap());
            }
        }
        Err(err) => {
            println!("Error getting rustc version: {}", err);
        }
    }
    return "unknown".to_string();
}

fn open(path: &str) -> File {
    match File::create(&Path::new(path)) {
        Ok(f) => f, Err(e) => panic!("file error: {}", e),
    }
}

pub fn main() {
    // Write the current Git HEAD hash into the version file.
    write!(&mut open("src/git_version.inc"), "{}", git_version()).unwrap();
    write!(&mut open("src/rustc_version.inc"), "{}", rustc_version()).unwrap();
}
