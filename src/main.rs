use clap::Command;
use simple_eyre::eyre::Report;

use crate::subcommands::add;

mod subcommands;
mod unix;
mod validation;

fn main() -> Result<(), Report> {
    simple_eyre::install()?;

    let matches = Command::new("quadeploy")
        .version("0.1.0")
        .about("Manages quadlets that are isolated by Unix users.")
        .subcommand(add::def())
        //TODO: ls/list
        //TODO: migrate
        //TODO: rm/remove
        .get_matches();
    
    match matches.subcommand() {
        Some(("add", args)) => add::exec(args),
        //TODO: ls/list
        //TODO: migrate
        //TODO: rm/remove
        _ => unreachable!("Exhausted list of subcommands.")
    }
}
