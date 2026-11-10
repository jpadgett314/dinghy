use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::BookmarkTag)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmark_tag/bookmark_tag.ui")]
    pub struct BookmarkTag {
        #[property(get, set)]
        pub tag_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkTag {
        const NAME: &'static str = "BookmarkTag";
        type Type = super::BookmarkTag;
        type ParentType = gtk::Button;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkTag {
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for BookmarkTag {}
    impl ButtonImpl for BookmarkTag {}
}

glib::wrapper! {
    pub struct BookmarkTag(ObjectSubclass<imp::BookmarkTag>)
        @extends gtk::Widget, gtk::Button,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

#[gtk::template_callbacks]
impl BookmarkTag {
    pub fn new(tag_name: &str) -> Self {
        glib::Object::builder()
            .property("tag-name", tag_name)
            .build()
    }

    #[template_callback]
    pub fn on_clicked(&self) {
        let label = self.imp().tag_name.borrow();
        let query = format!("#{}", label).to_variant();
        let _ = self.activate_action("bookmarks.search", Some(&query));
    }
}
