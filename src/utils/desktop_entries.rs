use arc_swap::ArcSwap;
use freedesktop_desktop_entry::{Iter, default_paths, get_languages_from_env};
use lazy_static::lazy_static;
use std::sync::Arc;

pub struct AppEntry {
    pub name: String,
    pub icon: String,
    pub exec: String,
}

fn parse_desktop_entries() -> Vec<AppEntry> {
    let term =
        xdg_utils::query_default_app("x-scheme-handler/terminal").unwrap_or("xterm".to_string());

    let locales = get_languages_from_env();

    let entries: Vec<AppEntry> = Iter::new(default_paths())
        .entries(Some(&locales))
        .filter(|e| !e.no_display())
        .map(|e| AppEntry {
            name: e.full_name(&locales).unwrap().to_string(),
            icon: e.icon().unwrap_or("application-x-executable").to_string(),
            exec: parse_exec(e.exec().unwrap_or(""), &term, e.terminal()),
        })
        .collect();

    entries
}

fn parse_exec(exec: &str, term: &str, is_term: bool) -> String {
    let clean_exec = exec.split('%').next().unwrap_or(exec).to_string();

    if is_term {
        return format!("{} -e {}", term, clean_exec);
    }

    clean_exec
}

lazy_static! {
    static ref ENTRIES: ArcSwap<Vec<AppEntry>> = ArcSwap::from(Arc::new(parse_desktop_entries()));
}

pub async fn update() {
    ENTRIES.store(Arc::new(parse_desktop_entries()));
}

pub fn get() -> Arc<Vec<AppEntry>> {
    ENTRIES.load_full()
}
