use clap::{crate_version, App, AppSettings, Arg};

pub fn new() -> App<'static> {
    let app = App::new("linfo")
        .about("Check if files are symbolic links or junctions.")
        .version(crate_version!())
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::ArgRequiredElseHelp);

    let files = Arg::new("files")
        .about("File(s) to check.")
        .required(true)
        .multiple(true);

    app.arg(files)
}
