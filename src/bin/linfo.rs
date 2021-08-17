#![feature(windows_file_type_ext)]
#![cfg(windows)]

use std::{fs, os::windows::fs::FileTypeExt, process};

use wink::app_linfo;

struct Cmd {
    files: Vec<String>,
}

impl Cmd {
    fn from_args() -> Self {
        let m = app_linfo().get_matches();
        let files: Vec<_> = m.values_of("files").unwrap().map(String::from).collect();
        Self { files }
    }

    fn run(&self) -> Result<(), std::io::Error> {
        for f in &self.files {
            if let Ok(true) = junction::exists(f) {
                if let Ok(original) = junction::get_target(f) {
                    println!("{}: directory junction to {}", f, original.display());
                } else {
                    println!("{}: directory junction", f);
                }
                continue;
            }

            let md = fs::symlink_metadata(f)?;

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
            } else {
                println!("{}: not a junction or a symbolic link", f);
            }
        }
        Ok(())
    }
}

fn main() {
    if let Err(e) = Cmd::from_args().run() {
        eprintln!("error: {}", e);
        process::exit(2);
    }
}
