use clap::{crate_version, Arg, Command};

pub fn build() -> Command<'static> {
    let app = Command::new("rust-cli 🔐 Rust CLI Template")
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("path")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Specify target file"),
        );

    app
}
