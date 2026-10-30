use std::cell::{Cell, OnceCell};
use std::sync::Mutex;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::BookmarkStorage;
use crate::models::BookmarkCardModel;
use crate::util::EpochCounter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "BookmarkFeedStatus")]
pub enum BookmarkFeedStatus {
    #[default]
    Loading,
    Matches,
    NoMatches,
    EmptyServer,
}

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BookmarkFeedModelState {
        pub bookmarks: BookmarkStorage,
        pub bookmark_list: Vec<BookmarkCardModel>,
        pub query: String,
        pub query_page: i32,
        pub settings: OnceCell<gio::Settings>,
    }

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::BookmarkFeedModel)]
    pub struct BookmarkFeedModel {
        pub state: Mutex<BookmarkFeedModelState>,
        #[property(get, builder(BookmarkFeedStatus::default()))]
        pub status: Cell<BookmarkFeedStatus>,
        #[property(get, set)]
        pub epoch_counter: OnceCell<EpochCounter>,
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
            self.state.lock().unwrap().bookmark_list.len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.state
                .lock()
                .unwrap()
                .bookmark_list
                .get(position as usize)
                .cloned()
                .map(|bookmark| bookmark.upcast::<glib::Object>())
        }
    }

    impl BookmarkFeedModel {
        pub fn fetch_query(&self, query: String) {
            {
                let mut state = self.state.lock().unwrap();

                state.query = query.clone();
                state.query_page = 0;
            };

            self.fetch_and_splice(true);
        }

        pub fn fetch_more(&self) {
            {
                let mut state = self.state.lock().unwrap();

                state.query_page += 1;
            };

            self.fetch_and_splice(false);
        }

        pub fn fetch_and_splice(&self, replace: bool) {
            if replace {
                self.set_status(BookmarkFeedStatus::Loading);
            }

            let (bookmarks, query, page, page_size) = {
                let state = self.state.lock().unwrap();

                (
                    state.bookmarks.clone(),
                    state.query.clone(),
                    state.query_page,
                    state
                        .settings
                        .get()
                        .map(|s| s.get::<u32>("max-bookmarks-per-query") as i32)
                        .unwrap_or(100),
                )
            };

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
                    model.imp().update_status();
                    return;
                };

                let bookmarks = matches.iter().map(BookmarkCardModel::new);

                let (index, taken, added) = {
                    let mut state = model.imp().state.lock().unwrap();

                    let added = bookmarks.len();
                    let count = state.bookmark_list.len();
                    let index = if replace { 0 } else { state.bookmark_list.len() };
                    let taken = state.bookmark_list.splice(index..count, bookmarks).count();

                    (index, taken, added)
                };

                model.items_changed(index as u32, taken as u32, added as u32);
                model.imp().update_status();
            });
        }

        fn set_status(&self, status: BookmarkFeedStatus) {
            if self.status.replace(status) != status {
                self.obj().notify_status();
            }
        }

        fn update_status(&self) {
            let status = {
                let state = self.state.lock().unwrap();
                if !state.bookmark_list.is_empty() {
                    BookmarkFeedStatus::Matches
                } else if state.query.is_empty() {
                    BookmarkFeedStatus::EmptyServer
                } else {
                    BookmarkFeedStatus::NoMatches
                }
            };
            self.set_status(status);
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkFeedModel {}
}

glib::wrapper! {
    pub struct BookmarkFeedModel(ObjectSubclass<imp::BookmarkFeedModel>)
        @implements gio::ListModel;
}

impl BookmarkFeedModel {
    pub fn new(api: BookmarkStorage, cfg: gio::Settings) -> Self {
        let object: BookmarkFeedModel = glib::Object::builder()
            .property("epoch-counter", &EpochCounter::new())
            .build();

        {
            let mut state = object.imp().state.lock().unwrap();

            state.bookmarks = api.clone();
            state.settings = OnceCell::from(cfg.clone());
        }

        object
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
        glib::Object::builder().build()
    }
}
