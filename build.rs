use std::{
	env,
	io::Error,
};

use clap_complete::{
	generate_to,
	shells::{
		Elvish,
		PowerShell,
	},
};

include!("src/lib.rs");

fn main() -> Result<(), Error> {
	let out_dir = match env::var("OUT_DIR") {
		Err(_) => return Ok(()),
		Ok(s) => s,
	};

	for (mut app, name) in [
		(app_ln(), "ln"),
		(app_rmlink(), "rmlink"),
		(app_linfo(), "linfo"),
	] {
		generate_to(PowerShell, &mut app, name, &out_dir)?;
		generate_to(Elvish, &mut app, name, &out_dir)?;
	}

	Ok(())
}
