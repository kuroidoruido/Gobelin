mod command;
mod config;
mod gobelin_opt;

use crate::command::fmt::format_files;
use crate::config::parse_config;
use crate::gobelin_opt::{Command, Opt};
use structopt::StructOpt;

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    let config = parse_config(opt.root)?;
    match opt.cmd {
        Command::Fmt { files, verbose } => format_files(&config, &files, verbose),
    }?;

    Ok(())
}
