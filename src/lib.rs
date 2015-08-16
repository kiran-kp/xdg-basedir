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

#[cfg(feature = "unstable")]
pub mod unstable;

mod env_path;

pub use error::*;

#[cfg(feature = "unstable")]
pub use unstable::*;

use std::ffi::OsString;
use std::env;
use std::path::PathBuf;

/// Get the data home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home_from_env<'a, F>(get_env_var: &'a F) -> Result<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    env_path::get_env_path_or_default(get_env_var, "XDG_DATA_HOME", ".local/share")
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
    env_path::get_env_paths_or_default(get_env_var, "XDG_DATA_DIRS", "/usr/local/share:/usr/share")
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
    env_path::get_env_path_or_default(get_env_var, "XDG_CONFIG_HOME", ".config")
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
    env_path::get_env_paths_or_default(get_env_var, "XDG_CONFIG_DIRS", "/etc/xdg")
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
    env_path::get_env_path_or_default(get_env_var, "XDG_CACHE_HOME", ".cache")
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
    env_path::get_env_path(get_env_var, "XDG_RUNTIME_DIR")
}

/// Get ```$XDG_RUNTIME_DIR``` if found in the environment.
///
/// Returns None if ```$XDG_RUNTIME_DIR``` is not set, in which case it is up to the application
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir() -> Option<PathBuf> {
    get_runtime_dir_from_env(&env::var_os)
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
