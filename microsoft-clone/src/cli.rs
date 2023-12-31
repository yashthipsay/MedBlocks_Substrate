use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add { 
        #[structopt()]
        text: String
    },
    Done {
        #[structopt()]
        position: usize
    },
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Microsoft clone app",
    about = "An clone of microsoft cli"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

