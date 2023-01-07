use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use log::{debug, info};

use crate::config::{APP_ID, PROFILE, VERSION};
use crate::window::RisiUpgradeWindow;

mod imp {
    use crate::widgets::init_widgets;

    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct RisiUpgrade {
        pub window: OnceCell<WeakRef<RisiUpgradeWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RisiUpgrade {
        const NAME: &'static str = "RisiUpgrade";
        type Type = super::RisiUpgrade;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for RisiUpgrade {}

    impl ApplicationImpl for RisiUpgrade {
        fn activate(&self) {
            debug!("AdwApplication<RisiUpgrade>::activate");
            self.parent_activate();
            let app = self.instance();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = RisiUpgradeWindow::new(&*app);
            self.window
                .set(window.downgrade())
                .expect("Window already set");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("AdwApplication<RisiUpgrade>::startup");
            self.parent_startup();
            let app = self.instance();

            // Send the default icon to the shell.
            // Note that some shells still prefer a .desktop file
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_gactions();
            app.setup_accels();
            init_widgets();
        }
    }

    impl GtkApplicationImpl for RisiUpgrade {}
    impl AdwApplicationImpl for RisiUpgrade {}
}

glib::wrapper! {
    pub struct RisiUpgrade(ObjectSubclass<imp::RisiUpgrade>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl RisiUpgrade {
    fn main_window(&self) -> RisiUpgradeWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    /// Setup global GActions. These are traditionally the global keyboard shortcuts (such as quit)
    /// or functions that should be able to be called anywhere in the app
    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        self.add_action_entries([action_quit]).unwrap();
    }

    /// Setup keyboard shortcut (accelerators)
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    pub fn run(&self) {
        info!("risiUpgrade ({})", APP_ID);
        info!("Version: {} {}", VERSION, PROFILE);

        ApplicationExtManual::run(self);
    }
}

impl Default for RisiUpgrade {
    fn default() -> Self {
        glib::Object::new::<Self>(&[
            ("application-id", &APP_ID),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/io/risi/Upgrade/")),
        ])
    }
}
