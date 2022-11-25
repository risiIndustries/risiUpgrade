use application::RisiUpgrade;
use config::RESOURCES_FILE;
use gtk::glib;
use gtk::{self, gio};

mod application;
mod config;
mod widgets;
mod window;

fn main() {
    // Init logging
    pretty_env_logger::init();

    glib::set_application_name("risiUpgrade");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = RisiUpgrade::default();
    app.run();
}
