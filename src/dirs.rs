use env_path::*;
use error::*;

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
    get_path_or_default(get_env_var, "XDG_DATA_HOME", ".local/share")
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
    get_paths_or_default(get_env_var, "XDG_DATA_DIRS", "/usr/local/share:/usr/share")
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
    get_path_or_default(get_env_var, "XDG_CONFIG_HOME", ".config")
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
    get_paths_or_default(get_env_var, "XDG_CONFIG_DIRS", "/etc/xdg")
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
    get_path_or_default(get_env_var, "XDG_CACHE_HOME", ".cache")
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
    get_path(get_env_var, "XDG_RUNTIME_DIR")
}

/// Get ```$XDG_RUNTIME_DIR``` if found in the environment.
///
/// Returns None if ```$XDG_RUNTIME_DIR``` is not set, in which case it is up to the application
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir() -> Option<PathBuf> {
    get_runtime_dir_from_env(&env::var_os)
}

