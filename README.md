# rusty-journal
My first Rust todo cli

Made by following the Microsoft tutorial [Build a command-line to-do list program](https://learn.microsoft.com/en-gb/training/modules/rust-create-command-line-program/)

## Dev

Use [bacon](https://crates.io/crates/bacon) to run and check code

## Run
`cargo run`

It creates a `.rusty-journal.json` file in home directory containing the dodo list in json

```
USAGE:
    rusty-journal [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-file <journal-file>    Use a different journal file

SUBCOMMANDS:
    add     Write tasks to the journal file
    done    Remove an entry from the journal file by position
    help    Prints this message or the help of the given subcommand(s)
    list    List all tasks in the journal file
```
