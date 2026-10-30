use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::Connection;
use crate::views::ConnectDialog;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::DinghyWelcome)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/welcome/welcome.ui")]
    pub struct DinghyWelcome {
        #[property(get, set, construct)]
        pub connection: RefCell<Option<Connection>>,
        #[property(get, set, construct)]
        pub settings: RefCell<Option<gio::Settings>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DinghyWelcome {
        const NAME: &'static str = "DinghyWelcome";
        type Type = super::DinghyWelcome;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for DinghyWelcome {
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for DinghyWelcome {}
    impl BoxImpl for DinghyWelcome {}
}

glib::wrapper! {
    pub struct DinghyWelcome(ObjectSubclass<imp::DinghyWelcome>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl DinghyWelcome {
    pub fn new(connection: &Connection, settings: &gio::Settings) -> Self {
        glib::Object::builder()
            .property("connection", connection)
            .property("settings", settings)
            .build()
    }

    #[template_callback]
    fn on_connect_instance_clicked(&self) {
        let Some(connection) = self.connection() else { return };
        let Some(settings) = self.settings() else {return};

        ConnectDialog::new(&connection, &settings).present(Some(self));
    }
}

