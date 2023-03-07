use structopt::StructOpt;
use std::path::PathBuf;
use anyhow;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".journal.json");
        path
    })
}

fn main()-> anyhow::Result<()>{
    // get the command line arguments
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    //unpack the journal file
    let journal_file = journal_file
    .or_else(find_default_journal_file)
    .expect("Failed to find journal file.");
    // perform the action
    match action {
        Add { task } => tasks::add_task(Task::new(task), journal_file),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}
