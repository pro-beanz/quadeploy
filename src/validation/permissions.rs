use simple_eyre::eyre::{
    Report,
    eyre
};
use users::{
    get_current_uid,
    get_group_by_gid,
    get_user_by_uid
};

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