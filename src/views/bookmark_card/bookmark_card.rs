use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::bookmark::Bookmark;
use crate::icon_cache::fetch_icon;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmark_card/bookmark_card.ui")]
    pub struct BookmarkCard {
        pub bookmark: RefCell<Bookmark>,
        #[template_child]
        pub url_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub title_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub description_label: gtk::TemplateChild<gtk::Label>,
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
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookmarkCard {}
    impl WidgetImpl for BookmarkCard {}
    impl BoxImpl for BookmarkCard {}
}

glib::wrapper! {
    pub struct BookmarkCard(ObjectSubclass<imp::BookmarkCard>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BookmarkCard {
    pub fn new(bookmark: &Bookmark) -> Self {
        let card: Self = glib::Object::new();

        card.set_data(bookmark);
        card
    }

    pub fn set_data(&self, bookmark: &Bookmark) {
        let imp = self.imp();

        imp.bookmark.replace(bookmark.clone());
        imp.url_label.set_label(&bookmark.url);
        imp.title_label.set_label(&bookmark.title);
        imp.description_label.set_label(&bookmark.description);

        self.set_tooltip_text(Some(&bookmark.url));

        let url = bookmark
            .favicon_url
            .as_ref()
            .filter(|url| !url.trim().is_empty());

        if let Some(url) = url {
            self.load_icon(url.clone());
        } else {
            self.set_placeholder_icon();
        }
    }

    pub fn set_placeholder_icon(&self) {
        let icon = &self.imp().icon_image;

        icon.set_icon_name(Some("user-bookmarks-symbolic"));
        icon.add_css_class("dim-label");
    }

    pub fn set_icon(&self, tex: &gdk::Texture) {
        let icon = &self.imp().icon_image;

        icon.set_paintable(Some(tex));
        icon.remove_css_class("dim-label");
    }

    pub fn load_icon(&self, url: String) {
        let weak_card = self.downgrade();

        glib::spawn_future_local(async move {
            let icon = gio::spawn_blocking(move || fetch_icon(&url)).await;

            let Some(card) = weak_card.upgrade() else {
                return;
            };

            if let Ok(Some(icon)) = icon {
                card.set_icon(&icon);
            } else {
                card.set_placeholder_icon();
            }
        });
    }

    /*
    pub fn open(&self) {
        let Some(bookmark) = &self.imp().bookmark.get() else {
            return;
        };

        let parent = self.root().and_downcast::<gtk::Window>();
        let opener = gtk::UriLauncher::new(&bookmark.url);

        opener.launch(
            parent.as_ref(),
            None::<&gio::Cancellable>,
            |result| {
                if let Err(error) = result {
                    eprintln!("Could not open bookmark: {error}");
                }
            }
        );
    }
    */
}

impl Default for BookmarkCard {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}

