use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    gio::Cancellable,
    glib::{self, SpawnFlags},
};
use vte4::{PtyFlags, TerminalExt, TerminalExtManual};
use log::error;
use subprocess::Exec;

use crate::config::DATA_DIR;

mod imp {
    use vte4::Terminal;

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, Default)]
    #[template(resource = "/io/risi/Upgrade/upgrader.ui")]
    pub struct Upgrader {
        #[template_child]
        pub vte: TemplateChild<Terminal>,
    }

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

    impl ObjectImpl for Upgrader {
        fn constructed(&self) {
            let obj = self.obj().clone();
            self.vte.connect_child_exited(move |_, exit_status| {
                obj.finish_upgrade(exit_status);
            });
        }
    }
    impl WidgetImpl for Upgrader {}
    impl BoxImpl for Upgrader {}
}

glib::wrapper! {
    pub struct Upgrader(ObjectSubclass<imp::Upgrader>)
        @extends gtk::Widget, gtk::Box;
}

impl Upgrader {
    pub fn run(&self) {
        let cancel = Cancellable::new();

        self.imp().vte.spawn_async(
            PtyFlags::DEFAULT,
            Some(DATA_DIR),
            &["./upgrade.sh"],
            &[],
            SpawnFlags::SEARCH_PATH,
            || {},
            -1,
            Some(&cancel),
            |terminal, pid, err| {
                if err.is_some() {
                    terminal.feed_child(
                        format!("Could not run upgrade process: {}", err.unwrap().message())
                            .as_bytes(),
                    );
                } else {
                    terminal.watch_child(pid);
                }
            },
        )
    }

    pub fn finish_upgrade(&self, status: i32) {
        match status {
            0 => {
                let root = self.clone();
                let window = root.upcast::<gtk::Widget>().root().unwrap().downcast::<gtk::Window>().unwrap();

                let dialog = adw::MessageDialog::new(Some(&window), Some("Upgrade Complete"), Some("A reboot is now required. Reboot now?"));

                dialog.add_response("confirm", "Reboot");
                dialog.add_response("close", "Close");
                dialog.connect_response(None, |obj, res| {
                    let window = obj.clone();

                    if res == "confirm" {
                        let _ = Exec::cmd("pkexec").args(&["dnf", "system-upgrade", "reboot"]).join();
                        window.upcast::<gtk::Window>().transient_for().unwrap().activate_action("app.quit", None).unwrap();
                    } else {
                        // Close on any invalid response
                        window.upcast::<gtk::Window>().transient_for().unwrap().activate_action("app.quit", None).unwrap();
                    }
                });

                dialog.present();
            }
            // I don't know why, but Vte returns 256 for a regular `1` exit code.
            // So, we'll use 256 here
            256 => {
                self.error("")
            },
            _ => unimplemented!("Only `0` and `1` are accepted exit codes. Any other exit code is unimplemented and will crash"),
        }
    }

    pub fn error(&self, msg: &str) {
        let root = self.clone();
        let window = root.upcast::<gtk::Widget>().root().unwrap().downcast::<gtk::Window>().unwrap();

        let dialog = adw::MessageDialog::new(Some(&window), Some("An error occurred during the upgrade"), Some(msg));

        dialog.add_response("close", "Close");
        dialog.connect_response(None, |obj, _| {
            let window = obj.clone();
            window.upcast::<gtk::Window>().transient_for().unwrap().activate_action("app.quit", None).unwrap();
        });

        dialog.present();
        error!("An error occurred during the upgrade: {}.", msg);
    }
}
