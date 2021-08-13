use std::env;

use clap_generate::{
    generate_to,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};
include!("src/lib.rs");

fn generate_completions(mut app: App<'static>, bin_name: &str) {
    app.set_bin_name(bin_name);
    let outdir = env::var("OUT_DIR").unwrap();
    generate_to::<Bash, _, _>(&mut app, bin_name, &outdir);
    generate_to::<Elvish, _, _>(&mut app, bin_name, &outdir);
    generate_to::<Fish, _, _>(&mut app, bin_name, &outdir);
    generate_to::<PowerShell, _, _>(&mut app, bin_name, &outdir);
    generate_to::<Zsh, _, _>(&mut app, bin_name, &outdir);
}

fn main() {
    for (app, name) in [
        (app_ln(), "ln"),
        (app_rmlink(), "rmlink"),
        (app_linfo(), "linfo"),
    ] {
        generate_completions(app, name);
    }
}
