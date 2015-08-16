#![cfg_attr(feature = "unstable", feature(libc, convert))]

//! xdg-basedir is a utility library to make conforming to the
//! [XDG basedir specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.
//!
//! #Example
//! ```ignore
//! #![cfg(unix)]
//! extern crate xdg_basedir;
//!
//! #![cfg(unix)]
//! use xdg_basedir::*;
//! use std::path::PathBuf;
//! ...
//! let data_home: PathBuf = try!(get_data_home());
//! ...
//! ```
//!
//! Alternate implementation and some initial source borrowed from [rust-xdg](https://github.com/o11c/rust-xdg).
//! The APIs provided by ```rust-xdg``` and ```xdg-basedir``` are different.

#[cfg(feature = "unstable")]
extern crate libc;

pub mod error;

pub use error::*;

use std::convert::From;
use std::env::{self, home_dir, split_paths};
use std::ffi::OsString;
use std::path::PathBuf;

#[cfg(feature = "unstable")]
use std::fs;
#[cfg(feature = "unstable")]
use std::os::unix::fs::PermissionsExt;
#[cfg(feature = "unstable")]
use std::path::Path;

/// Get the data home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home_from_env<'a, F>(get_env_var: &'a F) -> Result<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_path_or_default(get_env_var, "XDG_DATA_HOME", ".local/share")
}

/// Get the data home directory.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home() -> Result<PathBuf> {
    get_data_home_from_env(&env::var_os)
}

/// Get the default data directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs_from_env<'a, F>(get_env_var: &'a F) -> Vec<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_paths_or_default(get_env_var, "XDG_DATA_DIRS", "/usr/local/share:/usr/share")
}

/// Get the data directories.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs() -> Vec<PathBuf> {
    get_data_dirs_from_env(&env::var_os)
}

/// Get the config home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home_from_env<'a, F>(get_env_var: &'a F) -> Result<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_path_or_default(get_env_var, "XDG_CONFIG_HOME", ".config")
}
/// Get the config home directory.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home() -> Result<PathBuf> {
    get_config_home_from_env(&env::var_os)
}

/// Get the default config directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs_from_env<'a, F>(get_env_var: &'a F) -> Vec<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_paths_or_default(get_env_var, "XDG_CONFIG_DIRS", "/etc/xdg")
}

/// Get the config directories.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs() -> Vec<PathBuf> {
    get_config_dirs_from_env(&env::var_os)
}

/// Get the cache home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home_from_env<'a, F>(get_env_var: &'a F) -> Result<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_path_or_default(get_env_var, "XDG_CACHE_HOME", ".cache")
}

/// Get the cache home directory.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home() -> Result<PathBuf> {
    get_cache_home_from_env(&env::var_os)
}

/// Get ```$XDG_RUNTIME_DIR``` if found in the environment.
/// This method allows having a custom environment.
///
/// Returns None if ```$XDG_RUNTIME_DIR``` is not set, in which case it is up to the application
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir_from_env<'a, F>(get_env_var: &'a F) -> Option<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_path(get_env_var, "XDG_RUNTIME_DIR")
}

/// Get ```$XDG_RUNTIME_DIR``` if found in the environment.
///
/// Returns None if ```$XDG_RUNTIME_DIR``` is not set, in which case it is up to the application
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir() -> Option<PathBuf> {
    get_runtime_dir_from_env(&env::var_os)
}

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
#[cfg(feature = "unstable")]
pub fn test_runtime_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::metadata(&path)
        .or_else(|e| Err(Error::from(e)))
        .map(|attr| (attr.permissions().mode()))
        .and_then(inner::check_permissions)
        .and(inner::test_dir_uid_is_current_user(path.as_ref()))
}

/// Get path from environment variable's value or a default path relative to home_dir
fn get_env_path_or_default<'a, F>(get_env_var: &'a F, env_var: &'a str, default: &'a str) -> Result<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_path(get_env_var, env_var)
        .or(home_dir().map(|p| p.join(default)))
        .ok_or(Error::from(XdgError::NoHomeDir))
}

/// Get an environment variable's value as a PathBuf.
fn get_env_path<'a, F>(get_env_var: &'a F, env_var: &'a str) -> Option<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_var(env_var)
        .map(PathBuf::from)
        .into_iter()
        .filter(|x| x.is_absolute())
        .next()
}

fn get_env_paths_or_default<'a, F>(get_env_var: &'a F, env_var: &'a str, default: &'a str) -> Vec<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    let path_string = (*get_env_var)(env_var)
        .into_iter()
        .filter(|x| x != "")
        .next()
        .unwrap_or(OsString::from(default));

    split_paths(&path_string).collect()
}

#[cfg(feature = "unstable")]
mod inner {
    use super::*;

    use libc::funcs::posix88::stat_::stat;
    use libc::funcs::posix88::unistd::getuid;
    use libc::types::os::arch::posix01::stat as Stat;
    use std::ffi::CString;
    use std::mem;
    use std::path::Path;

    pub fn test_dir_uid_is_current_user(path: &Path) -> Result<()> {
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

    pub fn check_permissions(permissions: u32) -> Result<()> {
        match permissions == 0o700 {
            true => Ok(()),
            false => Result::from(XdgError::IncorrectPermissions)
        }
    }

    fn cstr(path: &Path) -> Result<CString> {
        path.as_os_str().to_cstring()
            .ok_or(Error::from(XdgError::InvalidPath))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::env::{self, home_dir, join_paths, split_paths};
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn test_env_with_no_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("dummy", "");

        let f = &|var: &str| { custom_env.get(var).map(OsString::from) };

        assert!(get_data_home_from_env(f).unwrap()   == home_dir().unwrap().join(".local/share"));
        assert!(get_data_dirs_from_env(f)            == vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")]);
        assert!(get_config_home_from_env(f).unwrap() == home_dir().unwrap().join(".config"));
        assert!(get_config_dirs_from_env(f)          == vec![PathBuf::from("/etc/xdg")]);
        assert!(get_cache_home_from_env(f).unwrap()  == home_dir().unwrap().join(".cache"));
        assert!(get_runtime_dir_from_env(f)          == None);
    }

    #[test]
    fn test_env_with_empty_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("XDG_DATA_HOME", "");
        custom_env.insert("XDG_DATA_DIRS", "");
        custom_env.insert("XDG_CONFIG_HOME", "");
        custom_env.insert("XDG_CONFIG_DIRS", "");
        custom_env.insert("XDG_CACHE_HOME", "");

        let f = &|var: &str| { custom_env.get(var).map(OsString::from) };

        assert!(get_data_home_from_env(f).unwrap()   == home_dir().unwrap().join(".local/share"));
        assert!(get_data_dirs_from_env(f)            == vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")]);
        assert!(get_config_home_from_env(f).unwrap() == home_dir().unwrap().join(".config"));
        assert!(get_config_dirs_from_env(f)          == vec![PathBuf::from("/etc/xdg")]);
        assert!(get_cache_home_from_env(f).unwrap()  == home_dir().unwrap().join(".cache"));
        assert!(get_runtime_dir_from_env(f)          == None);
    }

    #[test]
    fn test_env_with_xdg_vars() {
        let cwd = PathBuf::from(&env::current_dir().unwrap());
        let mut custom_env = HashMap::new();

        custom_env.insert("XDG_DATA_HOME", cwd.join("user/data").into_os_string());
        custom_env.insert("XDG_DATA_DIRS", join_paths(vec![cwd.join("share/data"), cwd.join("local/data")]).unwrap());
        custom_env.insert("XDG_CONFIG_HOME", cwd.join("user/config").into_os_string());
        custom_env.insert("XDG_CONFIG_DIRS", join_paths(vec![cwd.join("config"), cwd.join("local/config")]).unwrap());
        custom_env.insert("XDG_CACHE_HOME", cwd.join("user/cache").into_os_string());

        let f = &|var: &str| { custom_env.get(var).map(OsString::from) };

        assert!(get_data_home_from_env(f).unwrap()   == custom_env.get("XDG_DATA_HOME").map(PathBuf::from).unwrap());
        assert!(get_data_dirs_from_env(f)            == split_paths(&custom_env["XDG_DATA_DIRS"]).collect::<Vec<PathBuf>>());
        assert!(get_config_home_from_env(f).unwrap() == custom_env.get("XDG_CONFIG_HOME").map(PathBuf::from).unwrap());
        assert!(get_config_dirs_from_env(f)          == split_paths(&custom_env["XDG_CONFIG_DIRS"]).collect::<Vec<PathBuf>>());
        assert!(get_cache_home_from_env(f).unwrap()  == custom_env.get("XDG_CACHE_HOME").map(PathBuf::from).unwrap());
    }
}
