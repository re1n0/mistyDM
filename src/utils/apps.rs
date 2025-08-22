use crate::utils::history;
// use gio::{AppInfo, prelude::*};
use gtk::{
    Box, IconTheme, Image, Label, ListBoxRow, Orientation,
    gio::{AppInfo, prelude::*},
    prelude::*,
};
use std::collections::HashMap;

pub struct AppEntry {
    pub name: String,
    pub info: AppInfo,
    pub label: Label,
    pub score: i64,
    pub history: history::LaunchHistory,
}

pub fn load(history: &HashMap<String, history::LaunchHistory>) -> HashMap<ListBoxRow, AppEntry> {
    let mut entries: HashMap<ListBoxRow, AppEntry> = HashMap::new();
    let icon_theme = IconTheme::default();
    let apps = AppInfo::all();

    for app in apps {
        if !app.should_show() {
            continue;
        }

        let name = app.display_name().to_string();

        let id = match app.id() {
            Some(id) => id.to_string(),
            _ => continue,
        };

        let label = Label::builder()
            .xalign(0.0f32)
            .label(&name)
            .wrap(true)
            .build();

        let image = Image::builder().build();
        if let Some(icon) = app.icon()
            && icon_theme.has_gicon(&icon)
        {
            image.set_from_gicon(&icon);
        }

        let hbox = Box::builder().orientation(Orientation::Horizontal).build();
        hbox.prepend(&image);
        hbox.append(&label);

        let row = ListBoxRow::new();
        row.set_child(Some(&hbox));

        let history_data = history.get(&id).copied().unwrap_or_default();
        let last_used = history_data.last_used;
        let num_uses = history_data.num_uses;

        let app_entry: AppEntry = AppEntry {
            name,
            info: app,
            label,
            score: 100,
            history: history::LaunchHistory {
                last_used,
                num_uses,
            },
        };
        entries.insert(row, app_entry);
    }
    entries
}

/*
fn parse_exec(exec: &str, term: &str, is_term: bool) -> String {
    let clean_exec = exec.split('%').next().unwrap_or(exec).to_string();

    if is_term {
        return format!("{} -e {}", term, clean_exec);
    }

    clean_exec
}
*/
