use gtk::{Application, prelude::*};

mod ui;

fn main() {
    if let Ok(lang) = std::env::var("LANG") {
        rust_i18n::set_locale(lang.split('.').next().unwrap_or("en"));
    }

    smol::block_on(run_gui());
}

async fn run_gui() {
    let app = Application::builder().application_id(misty::APP_ID).build();

    app.connect_activate(ui::build);
    app.run();
}
