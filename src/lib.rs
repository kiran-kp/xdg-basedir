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

pub mod app_dirs;
pub mod dirs;
pub mod error;

#[cfg(feature = "unstable")]
pub mod unstable;

mod env_path;

pub use app_dirs::*;
pub use dirs::*;
pub use error::*;

#[cfg(feature = "unstable")]
pub use unstable::*;

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
