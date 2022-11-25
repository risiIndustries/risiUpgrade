use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, VariantTy},
};

use crate::application::RisiUpgrade;

mod imp {
    use adw::HeaderBar;
    use gtk::Stack;

    use crate::config::{APP_ID, PROFILE};

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/io/risi/Upgrade/window.ui")]
    pub struct RisiUpgradeWindow {
        #[template_child]
        pub headerbar: TemplateChild<HeaderBar>,
        #[template_child]
        pub stack: TemplateChild<Stack>,

        pub settings: gio::Settings,
    }

    impl Default for RisiUpgradeWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                stack: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RisiUpgradeWindow {
        const NAME: &'static str = "RisiUpgradeWindow";
        type Type = super::RisiUpgradeWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RisiUpgradeWindow {
        fn constructed(&self) {
            self.parent_constructed();

            if PROFILE == "development" {
                self.obj().add_css_class("devel");
            }

            self.obj().load_window_size();
            self.obj().setup_gactions();
        }
    }

    impl WidgetImpl for RisiUpgradeWindow {}

    impl WindowImpl for RisiUpgradeWindow {
        fn close_request(&self) -> gtk::Inhibit {
            if let Err(err) = self.instance().save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            self.parent_close_request()
        }
    }

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

    fn setup_gactions(&self) {
        let action_navigate = gio::SimpleAction::new("navigate", Some(VariantTy::STRING));

        let headerbar =  self.imp().headerbar.clone();
        let stack = self.imp().stack.clone();
        action_navigate.connect_activate(move |_, parameter| {
            if let Some(name) =
                parameter.map(|p| p.str().expect("Parameter must be of type String"))
            {
                headerbar.set_decoration_layout(Some(":minimize"));
                stack.set_visible_child_name(name);
            }
        });

        self.add_action(&action_navigate);
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = &self.imp().settings;

        let (width, height) = self.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = &self.imp().settings;

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
