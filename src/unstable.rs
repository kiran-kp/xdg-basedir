use error::*;

use libc::funcs::posix88::stat_::stat;
use libc::funcs::posix88::unistd::getuid;
use libc::types::os::arch::posix01::stat as Stat;

use std::ffi::CString;
use std::fs;
use std::mem;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Check that the value set for ```$XDG_RUNTIME_DIR``` is a valid path, has the correct owner and
/// permissions.
///
/// Returns Ok(()) if path is valid, owner is current uid and has permissions 0o700, or propogates any errors
/// that occurred.
///
/// >$XDG_RUNTIME_DIR defines the base directory relative to which user-specific non-essential runtime files and
/// other file objects (such as sockets, named pipes, ...) should be stored. The directory MUST be owned by the
/// user, and he MUST be the only one having read and write access to it. Its Unix access mode MUST be 0700.
/// >
/// >The lifetime of the directory MUST be bound to the user being logged in. It MUST be created when the user
/// first logs in and if the user fully logs out the directory MUST be removed. If the user logs in more than
/// once he should get pointed to the same directory, and it is mandatory that the directory continues to exist
/// from his first login to his last logout on the system, and not removed in between. Files in the directory
/// MUST not survive reboot or a full logout/login cycle.
/// >
/// >The directory MUST be on a local file system and not shared with any other system. The directory MUST by
/// fully-featured by the standards of the operating system. More specifically, on Unix-like operating systems
/// AF_UNIX sockets, symbolic links, hard links, proper permissions, file locking, sparse files, memory mapping,
/// file change notifications, a reliable hard link count must be supported, and no restrictions on the file name
/// character set should be imposed. Files in this directory MAY be subjected to periodic clean-up. To ensure that
/// your files are not removed, they should have their access time timestamp modified at least once every 6 hours
/// of monotonic time or the 'sticky' bit should be set on the file.
pub fn test_runtime_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::metadata(&path)
        .or_else(|e| Err(Error::from(e)))
        .map(|attr| (attr.permissions().mode()))
        .and_then(check_permissions)
        .and(test_dir_uid_is_current_user(path.as_ref()))
}

fn test_dir_uid_is_current_user(path: &Path) -> Result<()> {
    let p = try!(cstr(path));
    unsafe {
        let mut s: Stat = mem::zeroed();
        stat(p.as_ptr(), &mut s);

        let uid = getuid();

        match uid == s.st_uid {
            true => Ok(()),
            false => Result::from(XdgError::IncorrectOwner)
        }
    }
}

fn check_permissions(permissions: u32) -> Result<()> {
    match permissions == 0o700 {
        true => Ok(()),
        false => Result::from(XdgError::IncorrectPermissions)
    }
}

fn cstr(path: &Path) -> Result<CString> {
    path.as_os_str().to_cstring()
        .ok_or(Error::from(XdgError::InvalidPath))
}
