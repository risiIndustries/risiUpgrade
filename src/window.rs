use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use log::error;

use crate::application::RisiUpgrade;

mod imp {
    use adw::Carousel;
    use gtk::Button;
    use subprocess::Exec;

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, Default)]
    #[template(file = "/home/jamie/Projects/risiUpgrade/risiUpgrade.ui")]
    pub struct RisiUpgradeWindow {
        #[template_child]
        pub carousel: TemplateChild<Carousel>,
        #[template_child]
        pub next_button: TemplateChild<Button>,
        #[template_child]
        pub back_button: TemplateChild<Button>,
        #[template_child]
        pub backup_button: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RisiUpgradeWindow {
        const NAME: &'static str = "RisiUpgradeWindow";
        type Type = super::RisiUpgradeWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RisiUpgradeWindow {
        fn constructed(&self) {
            self.parent_constructed();

            self.backup_button.connect_clicked(move |_| {
                // Temporary fix
                if let Ok(_) = Exec::cmd("deja-dup").communicate() {
                    // Do things with the Communicator
                } else {
                    error!("Could not launch Deja Dup. Maybe open an error dialog?");
                }
            });

            let obj = self.obj().clone();
            self.next_button.connect_clicked(move |_| {
                obj.move_carousel_page(1);
            });

            let obj = self.obj().clone();
            self.back_button.connect_clicked(move |_| {
                obj.move_carousel_page(-1);
            });

            self.obj().set_button_state(self.carousel.position() as i32);
        }
    }

    impl WidgetImpl for RisiUpgradeWindow {}

    // A popular thing to do here is to save the window size on close
    impl WindowImpl for RisiUpgradeWindow {}

    impl ApplicationWindowImpl for RisiUpgradeWindow {}
    impl AdwApplicationWindowImpl for RisiUpgradeWindow {}
}

glib::wrapper! {
    pub struct RisiUpgradeWindow(ObjectSubclass<imp::RisiUpgradeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl RisiUpgradeWindow {
    pub fn new(app: &RisiUpgrade) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn move_carousel_page(&self, page_delta: i32) {
        let obj = self.imp();
        let current_page = obj.carousel.position() as i32;
        let pages = obj.carousel.n_pages() as i32;
        let new_page = (current_page + page_delta + pages) % pages;

        obj.carousel
            .scroll_to(&obj.carousel.nth_page(new_page.try_into().unwrap()), true);

        self.set_button_state(new_page);
    }

    /// Show or hide navigation buttons based on the current position
    pub fn set_button_state(&self, page: i32) {
        let obj = self.imp();

        // Pages starts on 1, Position starts on 0
        let pages = (obj.carousel.n_pages() as i32) - 1;
        let current_page = page;

        // Last page reached
        if current_page == pages {
            obj.next_button.set_visible(false);
        // First page
        } else if current_page == 0 {
            obj.back_button.set_visible(false);
        // Middle pages
        } else if current_page < pages {
            obj.next_button.set_visible(true);
            obj.back_button.set_visible(true);
        }
    }
}
