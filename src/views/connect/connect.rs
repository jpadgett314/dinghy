use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/connect/connect.ui")]
    pub struct ConnectDialog {}

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
    pub fn new() -> Self {
        glib::Object::new()
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        self.close();
    }
}

