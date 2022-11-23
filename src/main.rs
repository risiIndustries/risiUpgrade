use application::RisiUpgrade;
use gtk;
use gtk::{glib};

mod application;
mod config;
mod window;

fn main() {
    // Init logging
    pretty_env_logger::init();

    glib::set_application_name("risiUpgrade");

    let app = RisiUpgrade::default();
    app.run();
}
