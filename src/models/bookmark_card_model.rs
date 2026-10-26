use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::bookmark::Bookmark;
use crate::icon_cache::fetch_icon;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::BookmarkCardModel)]
    pub struct BookmarkCardModel {
        #[property(get, set)]
        pub url: RefCell<String>,
        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set)]
        pub description: RefCell<String>,
        #[property(get, set = Self::set_favicon_url, nullable)]
        pub favicon_url: RefCell<Option<String>>,
        #[property(get, set, nullable)]
        pub favicon_paintable: RefCell<Option<gdk::Paintable>>,
    }

    impl BookmarkCardModel {
        fn set_favicon_url(&self, url: String) {
            self.obj().load_icon(&url);
            self.favicon_url.replace(Some(url));
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkCardModel {
        const NAME: &'static str = "BookmarkCardModel";
        type Type = super::BookmarkCardModel;
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkCardModel {}
}

glib::wrapper! {
    pub struct BookmarkCardModel(ObjectSubclass<imp::BookmarkCardModel>);
}

impl BookmarkCardModel {
    pub fn new(bookmark: &Bookmark) -> Self {
        glib::Object::builder()
            .property("url", &bookmark.url)
            .property("title", &bookmark.title)
            .property("description", &bookmark.description)
            .property("favicon-url", &bookmark.favicon_url)
            .build()
    }

    pub fn open(&self, parent: Option<&gtk::Window>) {
        gtk::UriLauncher::new(&self.url()).launch(parent, None::<&gio::Cancellable>, |result| {
            if let Err(error) = result {
                eprintln!("Could not open bookmark: {error}");
            }
        });
    }

    pub fn load_icon(&self, favicon_url: &String) {
        let favicon_url_outer = favicon_url.clone();
        glib::spawn_future_local(
            glib::clone!(
                #[weak(rename_to = model)] self,
                async move {
                    let url = favicon_url_outer.clone();
                    let res = gio::spawn_blocking(move || fetch_icon(&url)).await;
                    if let Ok(icon) = res {
                        model.set_favicon_paintable(icon);
                    };
                },
            )
        );
    }
}
