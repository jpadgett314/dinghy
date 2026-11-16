use std::cell::{Cell, OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};
use linkding::ListBookmarksArgs;

use crate::api::{ApiError, Connection};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::ConnectDialog)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/connect/connect.ui")]
    pub struct ConnectDialog {
        #[property(get, set, construct_only, nullable)]
        pub connection: RefCell<Option<Connection>>,
        #[property(get, set, construct_only)]
        pub settings: OnceCell<gio::Settings>,
        #[property(get, set, builder(ApiError::None))]
        pub error: Cell<ApiError>,
        #[property(get, set)]
        pub pending: Cell<bool>,
        #[template_child]
        pub api_url_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub api_token_row: TemplateChild<adw::PasswordEntryRow>,
        #[template_child]
        pub connection_bindings: TemplateChild<glib::BindingGroup>,
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

            self.connection_bindings
                .bind("url", &*self.api_url_row, "text")
                .bidirectional()
                .sync_create()
                .build();
            self.connection_bindings
                .bind("token", &*self.api_token_row, "text")
                .bidirectional()
                .sync_create()
                .build();
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
            .property("error", ApiError::None)
            .build()
    }

    fn set_styles(&self, error: ApiError) {
        let tok_row = &self.imp().api_token_row;
        let url_row = &self.imp().api_url_row;

        // Rows manage own styles; do not bind to `css-classes`.
        url_row.remove_css_class("error");
        tok_row.remove_css_class("error");

        match error {
            ApiError::Rejected => tok_row.add_css_class("error"),
            ApiError::InvalidUrl => url_row.add_css_class("error"),
            ApiError::UnknownFailure => tok_row.add_css_class("error"),
            _ => (),
        }
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        self.close();
    }

    #[template_callback]
    fn on_submit_clicked(&self) {
        let Some(connection) = self.connection() else {
            return;
        };

        self.set_pending(true);

        let dialog = self.downgrade();

        glib::spawn_future_local(async move {
            let state = gio::spawn_blocking(move || {
                let request = ListBookmarksArgs {
                    query: Some(String::from("POLL")),
                    limit: Some(0),
                    offset: Some(0),
                    modified_since: None,
                };
                match connection.use_api(|api| api.list_bookmarks(request)) {
                    Ok(_) => ApiError::None,
                    Err(e) => ApiError::from(&e),
                }
            })
            .await;

            let state = state.unwrap_or(ApiError::UnknownFailure);
            let Some(dialog) = dialog.upgrade() else {
                return;
            };

            dialog.set_pending(false);
            dialog.set_styles(state);
            dialog.set_error(state);

            if state == ApiError::None {
                if dialog.settings().set_boolean("onboarded", true).is_ok() {
                    let _ = dialog.activate_action("app.onboard-complete", None);
                }
                dialog.close();
            }
        });
    }

    #[template_callback(function)]
    fn is_not(value: bool) -> bool {
        !value
    }

    #[template_callback(function)]
    fn is_error(error: ApiError) -> bool {
        error != ApiError::None
    }

    #[template_callback(function)]
    fn both(value1: bool, value2: bool) -> bool {
        value1 && value2
    }

    #[template_callback(function)]
    fn to_status_text(error: ApiError) -> String {
        match error {
            ApiError::Rejected => gettext("The server rejected this API token."),
            ApiError::Unreachable => gettext("This server is unreachable or unavailable."),
            ApiError::InvalidUrl => gettext("Enter a valid server URL, e.g. https://yourdomain.com:1234"),
            ApiError::UnknownFailure => gettext("The server rejected the connection."),
            _ => String::new(),
        }
    }
}
