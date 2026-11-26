use std::cell::{Cell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::glib;

use crate::models::BatchOperationModel;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::BatchToolbar)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/batch_toolbar/batch_toolbar.ui")]
    pub struct BatchToolbar {
        #[property(get, set, nullable)]
        pub operation: RefCell<Option<BatchOperationModel>>,
        #[property(get, set)]
        pub operation_active: Cell<bool>,
        #[property(get, set)]
        pub selected_count: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BatchToolbar {
        const NAME: &str = "BatchToolbar";
        type Type = super::BatchToolbar;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            BatchOperationModel::ensure_type();
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BatchToolbar {
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for BatchToolbar {}
    impl BoxImpl for BatchToolbar {}
}

glib::wrapper! {
    pub struct BatchToolbar(ObjectSubclass<imp::BatchToolbar>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl BatchToolbar {
    #[template_callback(function)]
    fn invert_bool(value: bool) -> bool {
        !value
    }

    #[template_callback(function)]
    fn selection_text(count: u32) -> String {
        gettext("%u selected").replace("%u", &count.to_string())
    }

    #[template_callback(function)]
    fn status_page(operation_active: bool) -> &'static str {
        if operation_active {
            "operation"
        } else {
            "selection"
        }
    }

    #[template_callback]
    fn on_delete_clicked(&self) {
        let _ = self.activate_action("bookmarks.delete-selected", None);
    }

    #[template_callback]
    fn on_close_clicked(&self) {
        let _ = self.activate_action("bookmarks.clear-selection", None);
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        let _ = self.activate_action("bookmarks.cancel-batch-operation", None);
    }
}
