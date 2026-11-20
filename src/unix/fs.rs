use std::{
    fs::{
        Permissions,
        create_dir,
        set_permissions
    },
    os::unix::fs::{
        PermissionsExt,
        chown
    },
    path::Path
};

use simple_eyre::eyre::{
    Report,
    eyre
};

use crate::{
    unix::permissions::PERMISSIONS, validation::{
        fs::directory_exists, permissions::{
            validate_exec_user,
            validate_gid,
            validate_uid
        }
    }
};

/**
 * Creates a directory with the given GID and UID. Validates that the user of this
 * command is running it as root for managing service users.
 */
pub fn create_directory(path: &str, uid: u32, gid: u32) -> Result<(), Report> {
    // Make the path absolute for simplicity's sake.
    let path = Path::new(path).canonicalize()?;

    // If the directory already exists, just return early.
    if directory_exists(&path, uid, gid)? {
        return Ok(())
    }

    // Verify that this command is running as root.
    validate_exec_user()?;

    // Verify that the UID and GID are valid.
    validate_uid(uid)?;
    validate_gid(gid)?;

    // Attempt to create the directory.
    let result = create_dir(&path);

    // Error handling.
    if !result.is_ok() {
        return Err(eyre!("Cannot create directory. Try running as sudo?"));
    }

    // Set ownership.
    let result = chown(&path, Some(uid), Some(gid));

    // Error handling.
    if !result.is_ok() {
        return Err(eyre!("Cannot set directory ownership. Try running as sudo?"));
    }

    // Set permissions.
    let result = set_permissions(path, Permissions::from_mode(PERMISSIONS::Directory as u32));

    // Error handling.
    if !result.is_ok() {
        return Err(eyre!("Cannot set directory permissions. Try running as sudo?"));
    }

    Ok(())
}
