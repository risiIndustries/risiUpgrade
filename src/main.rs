extern crate libnotify;

use application::RisiUpgrade;
use config::{RESOURCES_FILE, APP_ID};
use gtk::glib;
use gtk::{self, gio};

mod application;
mod config;
mod widgets;
mod window;

// Entrypoint: Parse CLI arguments
fn main() {
    let mut args = std::env::args();

    if args.nth(1).and_then(|value| {
        value.contains("--show_notification").then(|| {})
    }).is_some() {
        show_notification();
    } else {
        open_ui();
    }
}

fn open_ui() {
    // Init logging
    pretty_env_logger::init();

    glib::set_application_name("risiUpgrade");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = RisiUpgrade::default();
    app.run();
}

fn show_notification() {
    libnotify::init(&APP_ID).unwrap();

    let notif = libnotify::Notification::new("risiUpgrade",
                                         Some("An update is available for your system. Please open the risiUpgrade app for details"),
                                         None);
    notif.show().unwrap();
    libnotify::uninit();
}
