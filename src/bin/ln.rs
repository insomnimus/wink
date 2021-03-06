#![cfg(windows)]

use std::{
	fs,
	io,
	os::windows::fs as winfs,
	path::{
		Path,
		PathBuf,
	},
	process,
};

use normpath::PathExt;
use wink::app_ln;

enum LinkType {
	Symbolic,
	Hard,
	Junction,
	Infer,
}

struct Cmd {
	target: PathBuf,
	path: PathBuf,
	link_type: LinkType,
}

impl Cmd {
	fn from_args() -> io::Result<Self> {
		use LinkType::*;
		let m = app_ln().get_matches_from(wild::args());
		let path = m
			.value_of("path")
			.map(Path::new)
			.unwrap()
			.normalize_virtually()?
			.into_path_buf();
		let target = m
			.value_of("target")
			.map(Path::new)
			.unwrap()
			.normalize()?
			.into_path_buf();

		let link_type = if m.is_present("symbolic") {
			Symbolic
		} else if m.is_present("hard") {
			Hard
		} else if m.is_present("junction") {
			Junction
		} else {
			Infer
		};

		Ok(Self {
			path,
			target,
			link_type,
		})
	}

	fn run(mut self) -> io::Result<()> {
		if self.path.is_dir() {
			if let Some(name) = self.target.file_name() {
				self.path.push(name);
			}
		}
		match self.link_type {
			LinkType::Symbolic => {
				let md = fs::symlink_metadata(&self.target)?;
				if md.is_dir() {
					winfs::symlink_dir(&self.target, &self.path)
				} else {
					winfs::symlink_file(&self.target, &self.path)
				}
			}
			LinkType::Hard => fs::hard_link(&self.target, &self.path),
			LinkType::Junction => junction::create(&self.target, &self.path),
			LinkType::Infer => {
				let md = fs::symlink_metadata(&self.target)?;
				if md.is_dir() {
					winfs::symlink_dir(&self.target, &self.path)
				} else {
					fs::hard_link(&self.target, &self.path)
				}
			}
		}
	}
}

fn main() {
	if let Err(e) = Cmd::from_args().and_then(|x| x.run()) {
		eprintln!("error: {}", &e);
		process::exit(2);
	}
}
