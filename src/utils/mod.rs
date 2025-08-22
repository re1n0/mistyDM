use directories::ProjectDirs;
use std::path::PathBuf;
pub mod apps;
pub mod history;

fn get_proj_dirs() -> ProjectDirs {
    ProjectDirs::from(
        crate::APP_ID_PARTS[0],
        crate::APP_ID_PARTS[1],
        crate::APP_ID_PARTS[2],
    )
    .expect("Could not determine project directories")
}

pub fn get_config_file(file: &str) -> PathBuf {
    get_proj_dirs().config_dir().join(file)
}

pub fn get_cached_file(file: &str) -> PathBuf {
    get_proj_dirs().cache_dir().join(file)
}
