use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::views::ConnectDialog;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/welcome/welcome.ui")]
    pub struct DinghyWelcome {}

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
    #[template_callback]
    fn on_connect_instance_clicked(&self) {
        ConnectDialog::new().present(Some(self));
    }
}
