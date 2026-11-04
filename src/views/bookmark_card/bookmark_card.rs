use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, glib};

use crate::models::BookmarkCardModel;
use crate::views::BookmarkTag;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::BookmarkCard)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmark_card/bookmark_card.ui")]
    pub struct BookmarkCard {
        #[property(get, set, nullable)]
        pub model: RefCell<Option<BookmarkCardModel>>,
        #[property(get, set = Self::set_favicon, nullable)]
        pub favicon: RefCell<Option<gdk::Paintable>>,
        #[property(get, set = Self::set_tag_names)]
        pub tag_names: RefCell<glib::StrV>,
        #[template_child]
        pub tags_box: gtk::TemplateChild<adw::WrapBox>,
        #[template_child]
        pub icon_image: gtk::TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkCard {
        const NAME: &'static str = "BookmarkCard";
        type Type = super::BookmarkCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl BookmarkCard {
        fn remove_all_tags(&self) {
            while let Some(child) = self.tags_box.first_child() {
                if child.is::<BookmarkTag>() {
                    self.tags_box.remove(&child);
                } else {
                    break;
                }
            }
        }

        fn set_tag_names(&self, tag_names: glib::StrV) {
            self.remove_all_tags();
            for tag_name in &tag_names {
                let display_name = format!("{}", tag_name);
                self.tags_box.prepend(&BookmarkTag::new(&display_name));
            }
            self.tag_names.replace(tag_names);
        }

        fn set_favicon(&self, favicon: Option<gdk::Paintable>) {
            match favicon {
                None => self.set_favicon_placeholder(),
                Some(icon) => self.set_favicon_texture(&icon),
            }
        }

        fn set_favicon_placeholder(&self) {
            let icon = &self.icon_image;

            icon.set_icon_name(Some("user-bookmarks-symbolic"));
            icon.add_css_class("dim-label");
        }

        fn set_favicon_texture(&self, tex: &gdk::Paintable) {
            let icon = &self.icon_image;

            icon.set_paintable(Some(tex));
            icon.remove_css_class("dim-label");
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkCard {}

    impl WidgetImpl for BookmarkCard {}
    impl BoxImpl for BookmarkCard {}
}

glib::wrapper! {
    pub struct BookmarkCard(ObjectSubclass<imp::BookmarkCard>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl BookmarkCard {
    #[template_callback(function)]
    fn has_text(text: &str) -> bool {
        !text.is_empty()
    }

    #[template_callback]
    fn on_edit_clicked(&self) {
        let Some(model) = self.model() else { return };
        let id = model.id().to_variant();
        let _ = self.activate_action("bookmarks.edit-dialog", Some(&id));
    }
}

impl Default for BookmarkCard {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
