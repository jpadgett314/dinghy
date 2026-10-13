use adw::subclass::prelude::*;
use adw::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/preferences.ui")]
    pub struct PreferencesDialog {
        #[template_child]
        pub server_url_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub api_token_row: TemplateChild<adw::EntryRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesDialog {
        const NAME: &'static str = "PreferencesDialog";
        type Type = super::PreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PreferencesDialog {}
    impl WidgetImpl for PreferencesDialog {}
    impl AdwDialogImpl for PreferencesDialog {}
    impl PreferencesDialogImpl for PreferencesDialog {}
}

glib::wrapper! {
    pub struct PreferencesDialog(ObjectSubclass<imp::PreferencesDialog>)
        @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PreferencesDialog {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn bind_settings(&self, settings: &gio::Settings) {
        let imp = self.imp();

        settings
            .bind("server-url", &*imp.server_url_row, "text")
            .build();

        settings
            .bind("api-token", &*imp.api_token_row, "text")
            .build();
    }
}

