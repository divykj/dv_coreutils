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

#[cfg(test)]
mod test {
    use super::{echo, get_app, ARG_E, ARG_INPUTS, ARG_N};

    macro_rules! test_params {
        ($args:expr) => {{
            let app = get_app();
            let matches = app.get_matches_from($args);
            let mut buffer = Vec::new();
            let echo_result = echo(&matches, &mut buffer);

            (matches, echo_result, buffer)
        }};
    }

    #[test]
    fn no_args() {
        let (matches, echo_result, buffer) = test_params!(vec!["echo"]);

        assert!(matches.values_of(ARG_INPUTS).is_none());
        assert!(!matches.is_present(ARG_N));
        assert!(!matches.is_present(ARG_E));

        assert!(echo_result.is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "\n");
    }

    #[test]
    fn one_arg() {
        let (matches, echo_result, buffer) = test_params!(vec!["echo", "hello"]);

        assert!(matches.values_of(ARG_INPUTS).is_some(),);
        assert_eq!(
            matches
                .values_of(ARG_INPUTS)
                .unwrap()
                .collect::<Vec<&str>>(),
            vec!["hello"],
        );
        assert!(!matches.is_present(ARG_N));
        assert!(!matches.is_present(ARG_E));

        assert!(echo_result.is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "hello\n");
    }

    #[test]
    fn multiple_args() {
        let (matches, echo_result, buffer) = test_params!(vec!["echo", "hello", "there"]);

        assert!(matches.values_of(ARG_INPUTS).is_some(),);
        assert_eq!(
            matches
                .values_of(ARG_INPUTS)
                .unwrap()
                .collect::<Vec<&str>>(),
            vec!["hello", "there"],
        );
        assert!(!matches.is_present(ARG_N));
        assert!(!matches.is_present(ARG_E));

        assert!(echo_result.is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "hello there\n");
    }

    #[test]
    fn no_newline_arg() {
        let (matches, echo_result, buffer) = test_params!(vec!["echo", "-n", "hello", "there"]);

        assert!(matches.values_of(ARG_INPUTS).is_some(),);
        assert_eq!(
            matches
                .values_of(ARG_INPUTS)
                .unwrap()
                .collect::<Vec<&str>>(),
            vec!["hello", "there"],
        );
        assert!(matches.is_present(ARG_N));
        assert!(!matches.is_present(ARG_E));

        assert!(echo_result.is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "hello there");
    }

    #[test]
    fn unescape_arg() {
        let (matches, echo_result, buffer) = test_params!(vec!["echo", "-e", "hello\\tthere"]);

        assert!(matches.values_of(ARG_INPUTS).is_some(),);
        assert_eq!(
            matches
                .values_of(ARG_INPUTS)
                .unwrap()
                .collect::<Vec<&str>>(),
            vec!["hello\\tthere"],
        );
        assert!(!matches.is_present(ARG_N));
        assert!(matches.is_present(ARG_E));

        assert!(echo_result.is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "hello\tthere\n");
    }
}
