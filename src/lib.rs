use clap::{crate_version, App, AppSettings, Arg, ArgSettings};

pub fn app_linfo() -> App<'static> {
    let app = App::new("linfo")
        .about("Check if files are symbolic links or junctions.")
        .version(crate_version!())
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::ArgRequiredElseHelp);

    let files = Arg::new("files")
        .about("File(s) to check.")
        .required(true)
        .setting(ArgSettings::MultipleValues);

    app.arg(files)
}

pub fn app_ln() -> App<'static> {
    let app = App::new("ln")
	.about("Create links.")
	.version(crate_version!())
	.setting(AppSettings::ArgRequiredElseHelp)
	.setting(AppSettings::UnifiedHelpMessage)
	.after_long_help("If the type of the link is not specified, the default behaviour is to create hard links for files and soft links for directories.");

    let original = Arg::new("original")
        .about("The original file, for a link to be created to.")
        .required(true);

    let target = Arg::new("target")
        .about("The location of the link to be created.")
        .required(true);

    let symbolic = Arg::new("symbolic")
        .short('s')
        .long("symbolic")
        .visible_alias("soft")
        .about("Explicitly specify the type of the link as symbolic.")
        .group("type");

    let hard = Arg::new("hard")
        .short('h')
        .long("hard")
        .about("Explicitly specify the type of the link as hard.")
        .group("type");

    let junction = Arg::new("junction")
        .short('j')
        .long("junction")
        .about("Create a directory junction.")
        .group("type");

    app.arg(original)
        .arg(target)
        .arg(symbolic)
        .arg(hard)
        .arg(junction)
}

pub fn app_rmlink() -> App<'static> {
    let app = App::new("rmlink")
        .about("Removes links.")
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp);

    let files = Arg::new("files")
        .about("File(s) to unlink.")
        .setting(ArgSettings::MultipleValues);

    app.arg(files)
}
