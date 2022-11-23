use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

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

            self.backup_button.connect_clicked(|_| {
                // Temporary fix
                let _ = Exec::cmd("deja-dup")
                    .communicate()
                    .expect("Failed to run deja-dup");
            });

            let carousel = self.carousel.clone();
            self.next_button.connect_clicked(move |_| {
                carousel.scroll_to(&carousel.nth_page((carousel.position() + 1.0) as u32), true);
            });

            let carousel = self.carousel.clone();
            self.back_button.connect_clicked(move |_| {
                carousel.scroll_to(&carousel.nth_page((carousel.position() - 1.0) as u32), true);
            });
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
}
