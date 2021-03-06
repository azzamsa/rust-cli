use std::env;

use clap::{crate_name, crate_version, AppSettings, Arg, ColorChoice, Command};

pub fn build(interactive_output: bool) -> Command<'static> {
    let clap_color_choice = if interactive_output && env::var_os("NO_COLOR").is_none() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .arg_required_else_help(true)
        .color(clap_color_choice)
        .setting(AppSettings::DeriveDisplayOrder)
        .dont_collapse_args_in_usage(true)
        .about(
            "CLI 🏗️. \n
            Rust CLI Template",
        )
        .after_help(
            "Note: `cli -h` prints a short and concise overview while `cli --help` gives all \
                 details",
        )
        .arg(
            Arg::new("FILE")
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .empty_values(false)
                .help("File(s) to print"),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .overrides_with("color")
                .takes_value(true)
                .possible_values(&["auto", "never", "always"])
                .default_value("auto")
                .help("When to use colors (*auto*, never, always)")
                .long_help(
                    "Specify when to use colored output. The automatic mode \
                     only enables colors if an interactive terminal is detected \
                ",
                ),
        );

    app
}

#[test]
fn verify() {
    build(false).debug_assert()
}
