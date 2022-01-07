mod command;
mod config;
mod gobelin_opt;

use crate::command::add::month::add_month_file;
use crate::command::fmt::format_files;
use crate::command::init::init_directory;
use crate::command::update::update_all;
use crate::config::parse_config;
use crate::gobelin_opt::{AddCommand, Command, Opt};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    let cmd = opt
        .cmd
        .or(Some(Command::Update { verbose: false }))
        .unwrap();
    match cmd {
        Command::Add { add_cmd } => add_sub_command(opt.root, add_cmd),
        Command::Fmt { files, verbose } => {
            let (config, _) = parse_config(&opt.root)?;
            format_files(&config, &files, verbose)
        }
        Command::Init {
            accounts,
            locale,
            vscode,
            verbose,
        } => init_directory(&opt.root, &accounts, locale, vscode, verbose),
        Command::Update { verbose } => {
            let (config, _) = parse_config(&opt.root)?;
            update_all(&opt.root, &config, verbose)
        }
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
        } => add_month_file(&config, base_path, year, month, verbose),
    }
}
