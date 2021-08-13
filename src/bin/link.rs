#![cfg(windows)]

use std::{fs, io, os::windows::fs as winfs, process};

use wink::app_link;

enum LinkType {
    Symbolic,
    Hard,
    Junction,
    Infer,
}

struct Cmd {
    original: String,
    target: String,
    link_type: LinkType,
}

impl Cmd {
    fn from_args() -> Self {
        use LinkType::*;
        let m = app_link().get_matches();
        let original = m.value_of("original").map(String::from).unwrap();
        let target = m.value_of("target").map(String::from).unwrap();

        let link_type = if m.is_present("symbolic") {
            Symbolic
        } else if m.is_present("hard") {
            Hard
        } else if m.is_present("junction") {
            Junction
        } else {
            Infer
        };

        Self {
            original,
            target,
            link_type,
        }
    }

    fn run(&self) -> io::Result<()> {
        match self.link_type {
            LinkType::Symbolic => {
                let md = fs::symlink_metadata(&self.original)?;
                if md.is_dir() {
                    winfs::symlink_dir(&self.original, &self.target)
                } else {
                    winfs::symlink_file(&self.original, &self.target)
                }
            }
            LinkType::Hard => fs::hard_link(&self.original, &self.target),
            LinkType::Junction => junction::create(&self.original, &self.target),
            LinkType::Infer => {
                let md = fs::symlink_metadata(&self.original)?;
                if md.is_dir() {
                    winfs::symlink_dir(&self.original, &self.target)
                } else {
                    fs::hard_link(&self.original, &self.target)
                }
            }
        }
    }
}

fn main() {
    if let Err(e) = Cmd::from_args().run() {
        eprintln!("error: {}", &e);
        process::exit(2);
    }
}
