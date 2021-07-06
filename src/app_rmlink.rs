use clap::{crate_version, App, AppSettings, Arg};

pub fn new() -> App<'static> {
    let app = App::new("rmlink")
        .about("Removes links.")
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp);

    let files = Arg::new("files").about("File(s) to unlink.").multiple(true);

    app.arg(files)
}
