use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, template_callbacks, Button};
use log::error;
use subprocess::Exec;

mod imp {
    use adw::Carousel;

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/io/risi/Upgrade/welcome.ui")]
    pub struct Welcome {
        #[template_child]
        pub carousel: TemplateChild<Carousel>,
        #[template_child]
        pub next_button: TemplateChild<Button>,
        #[template_child]
        pub back_button: TemplateChild<Button>,
    }

    #[template_callbacks]
    impl Welcome {
        pub fn open_error(&self, title: &str, body: &str) {
            let root = self.instance().clone();
            let window = root.upcast::<gtk::Widget>().root().unwrap().downcast::<gtk::Window>().unwrap();

            let dialog = adw::MessageDialog::new(Some(&window), Some(&format!("An error occurred: {}", title)), Some(body));

            dialog.add_response("close", "Close");
            dialog.set_close_response("close");

            dialog.present();
            error!("{}, {}", title, body);
        }

        #[template_callback]
        fn open_backup_tool(&self, _: &Button) {
            // Temporary fix
            if let Ok(_) = Exec::cmd("deja-dup").communicate() {
                // Do things with the Communicator
            } else {
                self.open_error("Could not launch Deja Dup", "Please make sure it's installed and available.");
            }
        }

        #[template_callback]
        fn start_upgrade(&self, btn: &Button) {
            if let Err(err) = btn.activate_action("win.navigate", Some(&"upgrade".to_variant())) {
                self.open_error("Could not navigate to page", &err.message);
            }
            if let Err(err) = btn.activate_action("win.upgrade", None) {
                self.open_error("Could not initiate upgrade", &err.message);
            }
        }

        /// Show or hide navigation buttons based on the current position
        pub fn set_button_state(&self, page: i32) {
            // Pages starts on 1, Position starts on 0
            let pages = (self.carousel.n_pages() as i32) - 1;
            let current_page = page;

            // Last page reached
            if current_page == pages {
                self.next_button.set_visible(false);
            // First page
            } else if current_page == 0 {
                self.back_button.set_visible(false);
            // Middle pages
            } else if current_page < pages {
                self.next_button.set_visible(true);
                self.back_button.set_visible(true);
            }
        }
    }

    impl Default for Welcome {
        fn default() -> Self {
            Self {
                carousel: TemplateChild::default(),
                next_button: TemplateChild::default(),
                back_button: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Welcome {
        const NAME: &'static str = "Welcome";
        type Type = super::Welcome;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Welcome {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj().clone();
            self.next_button.connect_clicked(move |_| {
                obj.move_carousel_page(1);
            });

            let obj = self.obj().clone();
            self.back_button.connect_clicked(move |_| {
                obj.move_carousel_page(-1);
            });

            self.set_button_state(self.carousel.position() as i32);
        }
    }

    impl WidgetImpl for Welcome {}
    impl BoxImpl for Welcome {}
}

glib::wrapper! {
    pub struct Welcome(ObjectSubclass<imp::Welcome>)
        @extends gtk::Widget, gtk::Box;
}

impl Welcome {
    pub fn move_carousel_page(&self, page_delta: i32) {
        let obj = self.imp();
        let current_page = obj.carousel.position() as i32;
        let pages = obj.carousel.n_pages() as i32;
        let new_page = (current_page + page_delta + pages) % pages;

        obj.carousel
            .scroll_to(&obj.carousel.nth_page(new_page.try_into().unwrap()), true);

        obj.set_button_state(new_page);
    }
}
