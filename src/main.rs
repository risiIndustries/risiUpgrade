use gtk;
use gtk::prelude::*;
use gtk::{Builder, Button};

use adw::prelude::*;
use adw::{Application, ApplicationWindow, Carousel};

use subprocess::Exec;

fn main() {
    let app = Application::builder()
        .application_id("io.risi.upgrade")
        .build();

    gtk::init().expect("Failed to initialize GTK.");
    let ui_src = include_str!("/home/cameron/Documents/risiOS/risiUpgrade/risiUpgrade.ui");
    let builder = Builder::from_string(ui_src);

    let carousel: Carousel = builder.object("carousel").unwrap();
    let window: ApplicationWindow = builder.object("window").unwrap();
    let next_button: Button = builder.object("next_button").unwrap();
    let back_button: Button = builder.object("back_button").unwrap();
    let backup_button: Button = builder.object("backup_button").unwrap();

    app.connect_activate(move |app| {
        window.set_application(Some(app));
        window.present();
    });

    backup_button.connect_clicked(|_| {
        Exec::cmd("deja-dup")
            .communicate()
            .expect("Failed to run deja-dup");
    });
    
    let carousel_next = carousel.clone();
    let carousel_back = carousel.clone();

    next_button.connect_clicked(move |_| {
        carousel_next.scroll_to(&carousel_next.nth_page((carousel_next.position() + 1.0) as u32), true);
    });
    back_button.connect_clicked(move |_| {
        carousel_back.scroll_to(&carousel_back.nth_page((carousel_back.position() - 1.0) as u32), true);
    });
    
    
    app.run();
}