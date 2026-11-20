use std::{
    fs::metadata,
    os::unix::fs::MetadataExt,
    path::PathBuf
};

use simple_eyre::eyre::Report;

/**
 * Verifies whether a directory exists with the given UID and GID ownership.
 */
pub fn directory_exists(path: &PathBuf, uid: u32, gid: u32) -> Result<bool, Report> {
    let metadata = metadata(path)?;
    Ok(path.is_dir()
        && metadata.uid() == uid
        && metadata.gid() == gid)
}
