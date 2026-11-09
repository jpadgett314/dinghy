use std::cell::{Cell, OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::BookmarkStorage;
use crate::models::BookmarkCardModel;
use crate::util::EpochCounter;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::BookmarkFeedModel)]
    pub struct BookmarkFeedModel {
        pub bookmark_list: RefCell<Vec<BookmarkCardModel>>,
        #[property(get, set, nullable)]
        pub storage: RefCell<Option<BookmarkStorage>>,
        #[property(get, set, nullable)]
        pub settings: RefCell<Option<gio::Settings>>,
        #[property(get)]
        pub query: RefCell<String>,
        #[property(get)]
        pub query_page: Cell<i32>,
        #[property(get, set)]
        pub epoch_counter: OnceCell<EpochCounter>,
        #[property(get)]
        pub pending: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkFeedModel {
        const NAME: &str = "BookmarkFeedModel";
        type Type = super::BookmarkFeedModel;
        type ParentType = glib::Object;
        type Interfaces = (gio::ListModel,);
    }

    impl ListModelImpl for BookmarkFeedModel {
        fn item_type(&self) -> glib::Type {
            BookmarkCardModel::static_type()
        }

        fn n_items(&self) -> u32 {
            self.bookmark_list.borrow().len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.bookmark_list
                .borrow()
                .get(position as usize)
                .cloned()
                .map(|bookmark| bookmark.upcast::<glib::Object>())
        }
    }

    impl BookmarkFeedModel {
        pub fn fetch_query(&self, query: String) {
            self.query.replace(query);
            self.query_page.set(0);

            self.set_pending(true);
            self.fetch_and_splice(true);
        }

        pub fn fetch_more(&self) {
            self.query_page.set(self.query_page.get() + 1);

            self.fetch_and_splice(false);
        }

        pub fn fetch_and_splice(&self, replace: bool){
            let Some(bookmarks) = self.obj().storage() else {
                return;
            };
            if bookmarks.connection().is_none() {
                return;
            }

            let query = self.obj().query();
            let page = self.query_page.get();
            let page_size = self.get_page_size();

            let model = self.obj().clone();

            glib::spawn_future_local(async move {
                let epoch = model.epoch_counter().create();

                let reply =
                    gio::spawn_blocking(move || bookmarks.search(query, page, page_size))
                        .await;

                if !epoch.is_latest() {
                    // A more recent request is in-progress; do not update feed status.
                    return;
                }

                let Ok(matches) = reply else {
                    model.imp().set_pending(false);
                    return;
                };

                let bookmarks = matches.iter().map(BookmarkCardModel::new);

                let (index, taken, added) = {
                    let mut bookmark_list = model.imp().bookmark_list.borrow_mut();

                    let added = bookmarks.len();
                    let count = bookmark_list.len();
                    let index = if replace { 0 } else { count };
                    let taken = bookmark_list.splice(index..count, bookmarks).count();

                    (index, taken, added)
                };

                model.items_changed(index as u32, taken as u32, added as u32);
                model.imp().set_pending(false);
            });
        }

        fn set_pending(&self, pending: bool) {
            if self.pending.replace(pending) != pending {
                self.obj().notify_pending();
            }
        }

        fn get_page_size(&self) -> i32 {
            self.obj()
                .settings()
                .map(|s| s.get::<u32>("max-bookmarks-per-query") as i32)
                .unwrap_or(100)
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkFeedModel {
        fn constructed(&self) {
            self.parent_constructed();
            self.epoch_counter.get_or_init(EpochCounter::new);
        }
    }
}

glib::wrapper! {
    pub struct BookmarkFeedModel(ObjectSubclass<imp::BookmarkFeedModel>)
        @implements gio::ListModel;
}

impl BookmarkFeedModel {
    pub fn new(api: BookmarkStorage, cfg: gio::Settings) -> Self {
        glib::Object::builder()
            .property("storage", api)
            .property("settings", cfg)
            .property("epoch-counter", &EpochCounter::new())
            .build()
    }

    pub fn find_by_id(&self, id: i32) -> Option<BookmarkCardModel> {
        self.imp()
            .bookmark_list
            .borrow()
            .iter()
            .find(|bookmark| bookmark.id() == id)
            .cloned()
    }

    pub fn fetch_more(&self) -> () {
        self.imp().fetch_more();
    }

    pub fn fetch_query(&self, query: String) {
        self.imp().fetch_query(query);
    }
}

impl Default for BookmarkFeedModel {
    fn default() -> Self {
        glib::Object::new()
    }
}
