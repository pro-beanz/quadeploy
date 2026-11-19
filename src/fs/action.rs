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
    path::PathBuf
};

use simple_eyre::eyre::{
    Report,
    eyre
};

use crate::fs::validation::{
    validate_exec_user,
    validate_gid,
    validate_uid
};

// Standard user permissions.
pub enum PERMISSIONS {
    // Corresponds to directory drwxr-xr-x permissions.
    Directory = 0o40755,

    // Corresponds to file -rw-r--r-- permissions.
    File = 0o100644,

    // Corresponds to executable -rwxr-x-r-x permissions.
    Executable = 0o100755,
}

/**
 * Creates a directory with the given GID and UID. Validates that the user of this
 * command is running it as root for managing service users.
 */
pub fn create_directory(path: PathBuf, uid: u32, gid: u32) -> Result<(), Report> {
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
