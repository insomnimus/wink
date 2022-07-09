#![feature(windows_file_type_ext)]
#![cfg(windows)]

use std::{
	fs,
	io,
	os::windows::fs::FileTypeExt,
	path::Path,
	process,
};

use wink::app_linfo;

fn run() -> i32 {
	let m = app_linfo().get_matches_from(wild::args());
	let mut n = 0;
	for f in m.values_of("file").unwrap() {
		if let Err(e) = show_info(f) {
			eprintln!("{f}: error: {e}");
			n += 1;
		}
	}
	n
}

fn show_info<P: AsRef<Path>>(p: P) -> io::Result<()> {
	let p = p.as_ref();
	if let Ok(true) = junction::exists(p) {
		if let Ok(original) = junction::get_target(p) {
			println!(
				"{}: directory junction to {}",
				p.display(),
				original.display()
			);
		} else {
			println!("{}: broken directory junction", p.display());
		}
		return Ok(());
	}

	let md = fs::symlink_metadata(p)?;
	let ftype = md.file_type();
	if !ftype.is_symlink() {
		println!("{}: not a junction or a symbolic link", p.display());
		return Ok(());
	}

	let original = p.read_link()?;
	match fs::symlink_metadata(&original) {
		Ok(o_md) => {
			let o_type = o_md.file_type();
			if (ftype.is_symlink_dir() && (o_type.is_dir() || o_type.is_symlink_dir()))
				|| (ftype.is_symlink_file() && (o_type.is_file() || o_type.is_symlink_file()))
			{
				println!(
					"{}: symbolic link to the {} {}",
					p.display(),
					if o_type.is_symlink_dir() {
						"directory symbolic link"
					} else if o_type.is_symlink_file() {
						"file symbolic link"
					} else if o_type.is_dir() {
						"directory"
					} else {
						"file"
					},
					original.display(),
				);
			} else {
				println!(
					"{}: broken symbolic link to {}",
					p.display(),
					original.display()
				);
			}
		}
		_ => println!(
			"{}: broken symbolic link to {}",
			p.display(),
			original.display()
		),
	}

	Ok(())
}

fn main() {
	process::exit(run());
}
