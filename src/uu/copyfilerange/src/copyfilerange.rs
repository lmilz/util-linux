// This file is part of the uutils util-linux package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{builder::ValueParser, crate_version, value_parser, Arg, ArgAction, Command};
use uucore::{error::UResult, format_usage, help_about, help_usage};

const ABOUT: &str = help_about!("copyfilerange.md");
const USAGE: &str = help_usage!("copyfilerange.md");

mod options {
    pub const SOURCE: &str = "source";
    pub const DESTINATION: &str = "destination";
    pub const RANGE: &str = "range";
    pub const RANGE_FILE: &str = "ranges";
    pub const VERBOSE: &str = "verbose";
}

#[cfg(target_os = "linux")]
#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let _matches: clap::ArgMatches = uu_app().try_get_matches_from(args)?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let _matches: clap::ArgMatches = uu_app().try_get_matches_from(args)?;

    Err(uucore::error::USimpleError::new(
        1,
        "`copyfilerange` is available only on Linux.",
    ))
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new(options::VERBOSE)
                .short('v')
                .long("verbose")
                .help("verbose mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::SOURCE)
                .help("source file")
                .required(true)
                .value_parser(ValueParser::path_buf()),
        )
        .arg(
            Arg::new(options::DESTINATION)
                .help("destination file")
                .required(true)
                .value_parser(ValueParser::os_string()),
        )
        .arg(
            Arg::new(options::RANGE)
                .help("range(s) of the form source_offset:destination_offset:length")
                .num_args(1..)
                .required_unless_present(options::RANGE_FILE)
                .value_parser(ValueParser::os_string())
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new(options::RANGE_FILE)
                .short('r')
                .long("ranges")
                .help("read range(s) separated by newlines from this file")
                .value_name("file")
                .value_parser(ValueParser::os_string()),
        )
}
