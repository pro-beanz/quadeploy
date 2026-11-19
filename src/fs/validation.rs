use std::path::Path;

use simple_eyre::eyre::{
    Report,
    eyre
};
//TODO: Replace this library with direct syscalls or build/find a new library, as
//      this one is unmaintained.
use users::{
    get_current_uid,
    get_group_by_gid,
    get_user_by_uid
};

use crate::fs::action::create_directory;

/**
 * Validates the user running the command. This tool should be run as root because it 
 * involves user/permission management.
 */
pub fn validate_exec_user() -> Result<(), Report> {
    // Throw an error if this program is run as a non-root user.
    if get_current_uid() != 0 {
        return Err(eyre!("This command must be run as root. Try using sudo?"));
    }

    Ok(())
}

/**
 * Validates that a UID exists.
 */
pub fn validate_uid(uid: u32) -> Result<(), Report> {
    // Get the user by its UID.
    let result = get_user_by_uid(uid);

    // Verify that the user actually exists in the system.
    if result.is_none() {
        Err(eyre!("Invalid UID provided."))
    } else {
        Ok(())
    }
}

/**
 * Validates that a GID exists.
 */
pub fn validate_gid(gid: u32) -> Result<(), Report> {
    // Get the group by its GID.
    let result = get_group_by_gid(gid);

    // Verify that the group actually exists in the system.
    if result.is_none() {
        Err(eyre!("Invalid GID provided."))
    } else {
        Ok(())
    }
}

/**
 * Validates that a directory exists. If the create flag is set, the optional UID and
 * GID values will be used to create the directory.
 */
pub fn validate_directory(
    path: &str,
    create: bool,
    uid: Option<u32>,
    gid: Option<u32>
) -> Result<(), Report> {
    // Make the path absolute for simplicity's sake.
    let path = Path::new(path).canonicalize()?;

    if path.is_dir() {
        // All good, nothing to do here.
        Ok(())
    } else if create  {
        // Validate that the UID was provided.
        if uid.is_none() {
            return Err(eyre!("UID was not provided."));
        }

        // Validate that the GID was provided.
        if gid.is_none() {
            return Err(eyre!("GID was not provided."));
        }

        // Create the directory if the parameters call for it.
        create_directory(path, uid.unwrap(), gid.unwrap())
    } else {
        // Throw an error if the directory doesn't exist and creation isn't specified.
        Err(eyre!("{} does not exist.", path.display()))
    }
}
