use std::cell::OnceCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::Connection;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::ConnectDialog)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/connect/connect.ui")]
    pub struct ConnectDialog {
        #[property(get, set, construct_only)]
        pub connection: OnceCell<Connection>,
        #[property(get, set, construct_only)]
        pub settings: OnceCell<gio::Settings>,
        #[template_child]
        pub api_url_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub api_token_row: TemplateChild<adw::PasswordEntryRow>,
    }

    impl ConnectDialog {
        fn bind_settings(&self) {
            let settings = self.settings.get().unwrap();

            settings
                .bind("api-url", &*self.api_url_row, "text")
                .build();

            settings
                .bind("api-token", &*self.api_token_row, "text")
                .build();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConnectDialog {
        const NAME: &'static str = "ConnectDialog";
        type Type = super::ConnectDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ConnectDialog {
        fn constructed(&self) {
            self.parent_constructed();
            self.bind_settings();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for ConnectDialog {}
    impl AdwDialogImpl for ConnectDialog {}
}

glib::wrapper! {
    pub struct ConnectDialog(ObjectSubclass<imp::ConnectDialog>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ConnectDialog {
    pub fn new(connection: &Connection, settings: &gio::Settings) -> Self {
        glib::Object::builder()
            .property("connection", connection)
            .property("settings", settings)
            .build()
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        self.close();
    }

    #[template_callback]
    fn on_connect_clicked(&self) {
        if let Ok(_) = self.settings().set_boolean("onboarded", true) {
            let _ = self.activate_action("app.onboard-complete", None);
        }

        self.close();
    }
}

