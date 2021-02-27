use clap::{App, AppSettings, Arg, ArgMatches};
use snailquote::unescape;
use std::io::{self, stdout, Write};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const ARG_N: &str = "NO_NEWLINE";
const ARG_E: &str = "ESCAPE_STRING";
const ARG_INPUTS: &str = "INPUTS";

fn get_app<'a>() -> App<'a, 'a> {
    App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author(PKG_AUTHORS)
        .settings(&[
            AppSettings::AllowLeadingHyphen,
            AppSettings::DisableHelpFlags,
            AppSettings::DisableVersion,
        ])
        .arg(Arg::with_name(ARG_N).short("n").takes_value(false))
        .arg(Arg::with_name(ARG_E).short("e").takes_value(false))
        .arg(Arg::with_name(ARG_INPUTS).multiple(true))
}

fn echo<W: Write>(cli_args: &ArgMatches, mut buffer: W) -> io::Result<()> {
    let input: String = match cli_args.values_of(ARG_INPUTS) {
        Some(inputs) => {
            let input: String = inputs.collect::<Vec<&str>>().join(" ");
            if cli_args.is_present(ARG_E) {
                unescape(&format!("\"{}\"", &input)).unwrap()
            } else {
                input
            }
        }
        None => "".to_string(),
    };

    let line_ending: &str = if cli_args.is_present(ARG_N) { "" } else { "\n" };

    buffer.write_fmt(format_args!("{}{}", input, line_ending))
}

fn main() {
    echo(&get_app().get_matches(), stdout()).unwrap()
}
