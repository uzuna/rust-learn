use clap::{App, AppSettings, Arg, SubCommand};
use clap::{_clap_count_exprs, arg_enum};

arg_enum! {
    #[derive(Debug)]
    enum Format{
        Csv,
        Json,
    }
}

fn main() {
    let opts = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("SERVER")
                .short("s")
                .long("server")
                .value_name("URL")
                .help("server url")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("port").about("post logs, taking input from stdin"))
        .subcommand(
            SubCommand::with_name("get").about("get logs").arg(
                Arg::with_name("FORMAT")
                    .short("f")
                    .long("format")
                    .help("log format")
                    .takes_value(true)
                    // .possible_values(&["csv", "json"])
                    .possible_values(&Format::variants())
                    .case_insensitive(true),
            ),
        );
    let matches = opts.get_matches();

    match matches.subcommand() {
        ("get", sub_match) => {
            let format = sub_match
                .and_then(|m| m.value_of("FORMAT"))
                .map(|m| m.parse().unwrap())
                .unwrap();
            match format {
                Format::Csv => unimplemented!(),
                Format::Json => unimplemented!(),
            }
        }
        _ => unreachable!(),
    }
}
