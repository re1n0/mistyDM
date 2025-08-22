use gtk::{
    Application, ApplicationWindow, Box, Entry, EventControllerKey, ListBox, ListBoxRow,
    Orientation, PolicyType, ScrolledWindow, gio::AppInfo, glib, prelude::*,
};
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
pub use misty::{
    config,
    utils::{apps, history},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn build(app: &Application) {
    let config = config::get();
    let keymap = config::keymap_get();
    let term = AppInfo::default_for_type("x-scheme-handler/terminal", false);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(misty::APP_NAME)
        .resizable(false)
        .decorated(false)
        .default_width(config.width)
        .default_height(config.height)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_namespace(Some(misty::APP_NAME));

    let margins = [
        (Edge::Left, config.margin.left),
        (Edge::Right, config.margin.right),
        (Edge::Top, config.margin.top),
        (Edge::Bottom, config.margin.bottom),
    ];

    let anchors = [
        (Edge::Left, config.anchor.left),
        (Edge::Right, config.anchor.right),
        (Edge::Top, config.anchor.top),
        (Edge::Bottom, config.anchor.bottom),
    ];

    for (margin, state) in margins {
        window.set_margin(margin, state);
    }

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let vbox = Box::builder()
        .name(misty::ROOT_BOX_CLASS)
        .orientation(Orientation::Vertical)
        .build();

    let entry = Entry::builder().name(misty::SEARCH_ENTRY_CLASS).build();

    vbox.prepend(&entry);

    let scroll = ScrolledWindow::builder()
        .name(misty::SCROLL_CLASS)
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .build();

    vbox.append(&scroll);

    let listbox = ListBox::builder()
        .name(misty::LISTBOX_CLASS)
        .vexpand(true)
        .build();
    scroll.set_child(Some(&listbox));

    let map_clone = {
        let history_arc = history::get();
        let guard = history_arc.lock().expect("Failed to lock history");
        guard.clone()
    };

    let history = Rc::new(RefCell::new(&map_clone));
    let entries = Rc::new(RefCell::new(apps::load(&history.borrow())));

    for row in (&entries.borrow() as &HashMap<ListBoxRow, apps::AppEntry>).keys() {
        listbox.append(row);
    }

    let event_controller = EventControllerKey::new();

    event_controller.connect_key_pressed(move |_, key, _, mod_key| {
        match (mod_key, key) {
            (m, k) if m.contains(keymap.modifier) && k == keymap.close => {
                std::process::exit(0);
            }
            (m, k) if m.contains(keymap.modifier) && k == keymap.up => {
                todo!(":3");
            }
            _ => {}
        }
        glib::Propagation::Proceed
    });

    window.add_controller(event_controller.clone());

    window.set_child(Some(&vbox));
    window.present();
}
