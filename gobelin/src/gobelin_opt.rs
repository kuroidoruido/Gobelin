use libgobelin::Locale;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Gobelin",
    about = "Gobelin is a text based personal accounting system."
)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub root: Option<PathBuf>,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Fmt {
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
        #[structopt(short, long)]
        verbose: bool,
    },
    Init {
        #[structopt(long)]
        locale: Option<Locale>,
        #[structopt(short = "-a", long = "--account")]
        accounts: Vec<String>,
        #[structopt(short, long)]
        verbose: bool,
    },
}
