use clap::{
    ArgMatches,
    Command
};
use simple_eyre::eyre::Report;

use crate::{
    fs::validation::validate_directory,
    user::user::create_user
};

pub fn def() -> Command {
    Command::new("add")
        .about("Adds a quadlet and a user. User directories are put into /srv/quadlets by default. If /srv does not exist and is permissioned improperly, this command will fail to execute.")
        //TODO: add args and update the about part.
}

pub fn exec(_args: &ArgMatches) -> Result<(), Report> {
    // Verify that the /srv directory exists, and create it as root if it doesn't.
    validate_directory("/srv", true, Some(0), Some(0))?;

    // Verify that the quadeploy user exists, and create it if it doesn't.
    create_user("quadlet");
    unimplemented!();

    // Verify that the /srv/quadlet directory exists.
    unimplemented!();

    //TODO: figure out the rest
}
