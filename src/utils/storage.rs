use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::app::APP_NAME;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StoredStates {
    pub run_count: usize,
}

impl StoredStates {
    pub fn get_or_init() -> Self {
        get_or_init_local_storage(APP_NAME, Self::default)
    }

    pub fn save(&self) {
        match LocalStorage::set(APP_NAME, self) {
            Ok(_) => log::info!("Saved StoredStates"),
            Err(e) => log::error!("Error when saving StoredStates: {}", e),
        }
    }
}

fn get_or_init_local_storage<T, F>(key: &str, default: F) -> T
    where T: for<'de> Deserialize<'de> + Serialize + Clone, F: FnOnce() -> T
{
    match LocalStorage::get::<T>(key) {
        Ok(value) => value,
        Err(e) => {
            log::error!("error: {}", e);
            let default = default();
            LocalStorage::set(key, default.clone()).unwrap();
            default
        }
    }
}

impl Default for StoredStates {
    fn default() -> Self {
        Self { run_count: 0 }
    }
}

impl Drop for StoredStates {
    fn drop(&mut self) {
        match LocalStorage::set(APP_NAME, self) {
            Ok(_) => log::info!("Dropping and saved StoredStates"),
            Err(e) => log::error!("Error saving when dropping StoredStates: {}", e),
        }
    }
}