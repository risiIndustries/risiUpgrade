use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, Default)]
    #[template(resource = "/io/risi/Upgrade/upgrader.ui")]
    pub struct Upgrader {}

    #[glib::object_subclass]
    impl ObjectSubclass for Upgrader {
        const NAME: &'static str = "Upgrader";
        type Type = super::Upgrader;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Upgrader {}
    impl WidgetImpl for Upgrader {}
    impl BoxImpl for Upgrader {}
}

glib::wrapper! {
    pub struct Upgrader(ObjectSubclass<imp::Upgrader>)
        @extends gtk::Widget, gtk::Box;
}
