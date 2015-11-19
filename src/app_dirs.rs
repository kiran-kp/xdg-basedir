use dirs;
use error;
 
use std::convert::AsRef;
use std::path::PathBuf;

pub struct AppDirs {
    pub data_home: PathBuf,
    pub data_dirs: Vec<PathBuf>,
    pub config_home: PathBuf,
    pub config_dirs: Vec<PathBuf>,
    pub cache_home: PathBuf,
    pub runtime_dir: Option<PathBuf>
}

impl AppDirs {
    pub fn new<T: AsRef<str>>(app_name: T) -> error::Result<AppDirs> {
        let subdir = app_name.as_ref();
        Ok(AppDirs {
            data_home: try!(dirs::get_data_home()).join(subdir),
            data_dirs: dirs::get_data_dirs().iter().map(|x| x.join(subdir)).collect(),
            config_home: try!(dirs::get_config_home()).join(subdir),
            config_dirs: dirs::get_config_dirs().iter().map(|x| x.join(subdir)).collect(),
            cache_home: try!(dirs::get_cache_home()).join(subdir),
            runtime_dir: dirs::get_runtime_dir().map(|x| x.join(app_name.as_ref()))
        })
    }
}
