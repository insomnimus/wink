use clap::{
	arg,
	crate_version,
	App,
	AppSettings,
	Arg,
};

pub fn app_linfo() -> App<'static> {
	App::new("linfo")
		.about("Check if files are symbolic links or junctions.")
		.version(crate_version!())
		.setting(AppSettings::ArgRequiredElseHelp)
		.arg(
			Arg::new("file")
				.help("File to check.")
				.multiple_values(true)
				.required(true),
		)
}

pub fn app_ln() -> App<'static> {
	App::new("ln")
	.about("Create links.")
	.version(crate_version!())
	.setting(AppSettings::ArgRequiredElseHelp)
	.after_long_help("If the type of the link is not specified, the default behaviour is to create hard links for files and soft links for directories.")
	.args(&[
	arg!(-s --symbolic "Create a symbolic link.").visible_alias("soft").group("kind"),
	arg!(-h --hard "Create a hard link.").group("kind"),
	arg!(-j --junction "Create an NTFS directory junction.").group("kind"),
	Arg::new("target")
	.help("The original file the link will target.")
	.required(true),
	Arg::new("path")
	.help("The path of the newly created link.")
	.required(true),
	])
}

pub fn app_rmlink() -> App<'static> {
	App::new("rmlink")
		.about("Removes links.")
		.version(crate_version!())
		.setting(AppSettings::ArgRequiredElseHelp)
		.arg(
			Arg::new("file")
				.help("Any number of files to unlink.")
				.required(true)
				.multiple_values(true),
		)
}
