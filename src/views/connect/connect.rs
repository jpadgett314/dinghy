use std::cell::OnceCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::models::Connection;

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
        let url = self.imp().api_url_row.text();
        let token = self.imp().api_token_row.text();

        self.connection().set_api(url.as_str(), token.as_str());

        if let (Ok(_), Ok(_)) = (
            self.settings().set_string("api-url", url.as_str()),
            self.settings().set_string("api-token", token.as_str())
        ) {
            if let Ok(_) = self.settings().set_boolean("onboarded", true) {
                let _ = self.activate_action("app.onboard-complete", None);
            }
        }

        self.close();
    }
}

