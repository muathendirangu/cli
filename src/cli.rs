use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    ///Write tasks to the journal file
    Add {
        ///The task description text
        #[structopt()]
        task: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cli",
    about = "A CLI tool to for keeping track of the  to-do written with Rust",
    author = "Muathe Ndirangu",
    version = "0.0.1"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
