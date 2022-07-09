use clap::{
	arg,
	crate_version,
	Command,
};

pub fn app_linfo() -> Command<'static> {
	Command::new("linfo")
		.about("Check if files are symbolic links or junctions.")
		.version(crate_version!())
		.arg_required_else_help(true)
		.arg(arg!(<file> ... "The file to check"))
}

pub fn app_ln() -> Command<'static> {
	Command::new("ln")
	.about("Create links.")
	.version(crate_version!())
	.arg_required_else_help(true)
	.after_long_help("If the type of the link is not specified, the default behaviour is to create hard links for files and soft links for directories.")
	.args(&[
	arg!(-s --symbolic "Create a symbolic link.").visible_alias("soft").group("kind"),
	arg!(-h --hard "Create a hard link.").group("kind"),
	arg!(-j --junction "Create an NTFS directory junction.").group("kind"),
	arg!(<target> "The original file the link will target."),
	arg!(<path> "The Path of the link to be created"),
	])
}

pub fn app_rmlink() -> Command<'static> {
	Command::new("rmlink")
		.about("Removes links.")
		.version(crate_version!())
		.arg_required_else_help(true)
		.arg(arg!(<file> ... "Files to unlink."))
}
