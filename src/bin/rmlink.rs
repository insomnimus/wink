#![feature(windows_file_type_ext)]
#![cfg(windows)]

use std::{
	fmt,
	fs,
	io,
	os::windows::fs::FileTypeExt,
	process,
};

use wink::app_rmlink;

enum Error {
	Io(io::Error),
	NotALink,
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Io(e) => write!(f, "{}", e),
			Self::NotALink => write!(f, "file is not a link"),
		}
	}
}

struct Cmd {
	files: Vec<String>,
}

impl Cmd {
	fn from_args() -> Self {
		let m = app_rmlink().get_matches_from(wild::args());

		let files: Vec<_> = m.values_of("file").unwrap().map(String::from).collect();

		Self { files }
	}

	fn run(&self) -> i32 {
		let mut exit_code = 0;
		for p in &self.files {
			if let Err(e) = unlink(p) {
				exit_code = 2;
				eprintln!("error unlinking {}: {}", p, &e);
			}
		}
		exit_code
	}
}

fn unlink(p: &str) -> Result<(), Error> {
	// first check if it's a junction
	if let Ok(true) = junction::exists(p) {
		return junction::delete(p).map_err(Error::from);
	}

	let md = fs::symlink_metadata(p)?;
	// the readonly attribute has to be removed before deletion
	let mut perms = md.permissions();
	if perms.readonly() {
		perms.set_readonly(false);
		fs::set_permissions(p, perms)?;
	}
	let ftype = md.file_type();
	if ftype.is_symlink_dir() {
		// dir symlinks need to be removed by remove_dir
		fs::remove_dir(p).map_err(Error::from)
	} else if ftype.is_symlink_file() {
		fs::remove_file(p).map_err(Error::from)
	} else {
		Err(Error::NotALink)
	}
}

fn main() {
	process::exit(Cmd::from_args().run())
}
