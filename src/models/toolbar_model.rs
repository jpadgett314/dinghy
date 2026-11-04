use std::cell::{Cell, RefCell};

use gtk::{glib, prelude::*, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::ToolbarModel)]
    pub struct ToolbarModel {
        #[property(get, set = Self::set_query, explicit_notify)]
        pub query: RefCell<String>,
        #[property(get, set = Self::set_searchbar_open, explicit_notify)]
        pub searchbar_open: Cell<bool>,
    }

    impl ToolbarModel {
        fn set_query(&self, query: String) {
            if *self.query.borrow() == query {
                return;
            }
            self.query.replace(query);
            self.obj().notify_query();
        }

        fn set_searchbar_open(&self, open: bool) {
            if self.searchbar_open.replace(open) != open {
                self.obj().notify_searchbar_open();
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ToolbarModel {
        const NAME: &'static str = "ToolbarModel";
        type Type = super::ToolbarModel;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ToolbarModel {}
}

glib::wrapper! {
    pub struct ToolbarModel(ObjectSubclass<imp::ToolbarModel>);
}

impl Default for ToolbarModel {
    fn default() -> Self {
        glib::Object::new()
    }
}
