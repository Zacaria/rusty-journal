use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::{
    cli::{Action, CommandLineArgs},
    tasks::Task,
};

mod cli;
mod tasks;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    //println!("{:#?}", cli::CommandLineArgs::from_args());

    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Action::Add { task } => tasks::add_task(journal_file, Task::new(task)),
        Action::Done { position } => tasks::complete_task(journal_file, position),
        Action::List => tasks::list_tasks(journal_file),
    }?;
    Ok(())
}
