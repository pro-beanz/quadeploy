use simple_eyre::eyre::Report;

/**
 * Standard user permissions.
 */
pub enum PERMISSIONS {
    // Corresponds to directory drwxr-xr-x permissions.
    Directory = 0o40755,

    // Corresponds to file -rw-r--r-- permissions.
    File = 0o100644,

    // Corresponds to executable -rwxr-x-r-x permissions.
    Executable = 0o100755,
}

pub fn create_user(username: &str) -> Result<(), Report> {
    //TODO: implement this lol
    unimplemented!()
}
