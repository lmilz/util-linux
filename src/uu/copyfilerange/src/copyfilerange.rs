// This file is part of the uutils util-linux package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{crate_version, Arg, ArgAction, Command};
use uucore::{error::UResult, format_usage, help_about, help_usage};

const ABOUT: &str = help_about!("copyfilerange.md");
const USAGE: &str = help_usage!("copyfilerange.md");

#[cfg(target_os = "linux")]
#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    Ok(())
}

pub fn uu_app() -> Command {
    Command::new("copyfilerange")
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(USAGE)
        .infer_long_args(true)
}
