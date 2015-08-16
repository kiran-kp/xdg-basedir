use error::*;

use std::convert::From;
use std::env::{home_dir, split_paths};
use std::ffi::OsString;
use std::path::PathBuf;

/// Get path from environment variable's value or a default path relative to home_dir
pub fn get_env_path_or_default<'a, F>(get_env_var: &'a F, env_var: &'a str, default: &'a str) -> Result<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_path(get_env_var, env_var)
        .or(home_dir().map(|p| p.join(default)))
        .ok_or(Error::from(XdgError::NoHomeDir))
}

/// Get an environment variable's value as a PathBuf.
pub fn get_env_path<'a, F>(get_env_var: &'a F, env_var: &'a str) -> Option<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    get_env_var(env_var)
        .map(PathBuf::from)
        .into_iter()
        .filter(|x| x.is_absolute())
        .next()
}

pub fn get_env_paths_or_default<'a, F>(get_env_var: &'a F, env_var: &'a str, default: &'a str) -> Vec<PathBuf>
    where F: Fn(&'a str) -> Option<OsString>
{
    let path_string = (*get_env_var)(env_var)
        .into_iter()
        .filter(|x| x != "")
        .next()
        .unwrap_or(OsString::from(default));

    split_paths(&path_string).collect()
}

