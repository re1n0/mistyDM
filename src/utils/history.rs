use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Default, Deserialize, Serialize, Clone, Copy)]
pub struct LaunchHistory {
    pub last_used: u64,
    pub num_uses: u32,
}

impl PartialEq for LaunchHistory {
    fn eq(&self, other: &Self) -> bool {
        self.last_used.eq(&other.last_used) && self.num_uses.eq(&other.num_uses)
    }
}

pub fn load(days: u32) -> HashMap<String, LaunchHistory> {
    let history_file = crate::utils::get_cached_file(crate::HISTORY_FILE);
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Gamma timeline split off");

    let history_str = if history_file.exists() {
        std::fs::read_to_string(history_file).expect("Failed to read the history file")
    } else {
        "".to_owned()
    };

    let cutoff = epoch.as_secs() - (days as u64) * 86400;

    let mut history = toml::from_str(&history_str).unwrap_or_else(|err| {
        eprintln!("Cannot parse history file: {}", err);
        HashMap::new()
    });

    history.retain(|_, data: &mut LaunchHistory| days == 0 || data.last_used >= cutoff);

    history
}

lazy_static! {
    static ref HISTORY: Arc<Mutex<HashMap<String, LaunchHistory>>> = Arc::new(Mutex::new(load(32)));
}

pub fn get() -> Arc<Mutex<HashMap<String, LaunchHistory>>> {
    HISTORY.clone()
}

pub async fn update(id: &str) {
    let history_arc = get();
    let mut history = history_arc.lock().expect("Failed to lock history");

    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Gamma timeline split off");

    let num_uses = history.get(id).map_or(0, |h| h.num_uses) + 1;

    history.insert(
        id.to_string(),
        LaunchHistory {
            last_used: epoch.as_secs(),
            num_uses,
        },
    );
}

pub async fn save() {
    let map_clone = {
        let history_arc = get();
        let guard = history_arc.lock().expect("Failed to lock history");
        guard.clone()
    };

    let history_file = crate::utils::get_cached_file(crate::HISTORY_FILE);
    let mut file = File::create(history_file).expect("Failed to open the history file for writing");

    let text = toml::to_string(&map_clone).expect("Failed to serialize history to TOML");

    file.write_all(text.as_bytes())
        .expect("Failed to write to the history file");
}
