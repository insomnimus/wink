#![feature(windows_file_type_ext)]
#![cfg(windows)]

mod app_linfo;

use std::os::windows::fs::FileTypeExt;
use std::{fs, process};

struct Cmd {
    files: Vec<String>,
}

impl Cmd {
    fn from_args() -> Self {
        let m = app_linfo::new().get_matches();
        let files: Vec<_> = m.values_of("files").unwrap().map(String::from).collect();
        Self { files }
    }

    fn run(&self) {
        for f in &self.files {
            let md = fs::symlink_metadata(f).unwrap_or_else(|e| {
                eprintln!("error accessing {}: {}", f, &e);
                process::exit(2);
            });

            let ftype = md.file_type();
            if ftype.is_symlink_dir() {
                if let Ok(original) = fs::canonicalize(f) {
                    println!(
                        "{}: symbolic link to the directory {}",
                        f,
                        original.display()
                    );
                } else {
                    println!("{}: symbolic link to a directory", f);
                }
            } else if ftype.is_symlink_file() {
                if let Ok(original) = fs::canonicalize(f) {
                    println!("{}: symbolic link to the file {}", f, original.display());
                } else {
                    println!("{}: symbolic link to a file", f);
                }
            } else if let Ok(true) = junction::exists(f) {
                if let Ok(original) = junction::get_target(f) {
                    println!("{}: directory junction to {}", f, original.display());
                } else {
                    println!("{}: directory junction", f);
                }
            } else {
                println!("{}: not a junction or a symbolic link", f);
            }
        }
    }
}

fn main() {
    Cmd::from_args().run();
}
