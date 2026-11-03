use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::TagChip)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/widgets/tag_chip/tag_chip.ui")]
    pub struct TagChip {
        #[property(get, set, construct_only)]
        pub tag_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TagChip {
        const NAME: &'static str = "TagChip";
        type Type = super::TagChip;
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
    impl ObjectImpl for TagChip {
        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WidgetImpl for TagChip {}
    impl BoxImpl for TagChip {}
}

glib::wrapper! {
    pub struct TagChip(ObjectSubclass<imp::TagChip>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl TagChip {
    pub fn new(tag_name: &str) -> Self {
        glib::Object::builder()
            .property("tag-name", tag_name)
            .build()
    }

    #[template_callback]
    fn on_remove_clicked(&self) {
        let _ = self.activate_action("tag-entry.remove", Some(&self.tag_name().to_variant()));
    }
}
