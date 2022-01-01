mod command;
mod config;
mod gobelin_opt;

use crate::command::add::month::add_month_file;
use crate::command::fmt::format_files;
use crate::command::init::init_directory;
use crate::config::parse_config;
use crate::gobelin_opt::{AddCommand, Command, Opt};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Add { add_cmd } => add_sub_command(opt.root.clone(), add_cmd),
        Command::Fmt { files, verbose } => {
            let (config, _) = parse_config(&opt.root)?;
            format_files(&config, &files, verbose)
        }
        Command::Init {
            accounts,
            locale,
            verbose,
        } => init_directory(&opt.root, &accounts, locale, verbose),
    }?;

    Ok(())
}

fn add_sub_command(base_path: Option<PathBuf>, cmd: AddCommand) -> Result<(), String> {
    let (config, base_path) = parse_config(&base_path)?;
    match cmd {
        AddCommand::Month {
            year,
            month,
            verbose,
        } => add_month_file(&config, base_path.clone(), year, month, verbose),
    }
}
