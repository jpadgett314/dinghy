use std::cell::OnceCell;
use std::sync::Mutex;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::bookmark_storage::BookmarkStorage;
use crate::views::BookmarkCard;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BookmarkPaginationState {
        pub bookmarks: BookmarkStorage,
        pub bookmark_cards: Vec<BookmarkCard>,
        pub query: String,
        pub query_page: i32,
        pub settings: OnceCell<gio::Settings>,
    }

    #[derive(Debug, Default)]
    pub struct BookmarkPagination {
        pub state: Mutex<BookmarkPaginationState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkPagination {
        const NAME: &str = "BookmarkPagination";
        type Type = super::BookmarkPagination;
        type ParentType = glib::Object;
        type Interfaces = (gio::ListModel,);
    }

    impl ListModelImpl for BookmarkPagination {
        fn item_type(&self) -> glib::Type {
            BookmarkCard::static_type()
        }

        fn n_items(&self) -> u32 {
            self.state
                .lock()
                .unwrap()
                .bookmark_cards.len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.state
                .lock()
                .unwrap()
                .bookmark_cards
                .get(position as usize)
                .cloned()
                .map(|card| card.upcast::<glib::Object>())
        }
    }

    impl BookmarkPagination {
        pub fn fetch_query(&self, query: String) {
            {
                let mut state = self.state.lock().unwrap();

                state.query = query.clone();
                state.query_page = 0;
            };

            self.fetch_and_splice(0);
        }

        pub fn fetch_more(&self) {
            let position = {
                let mut state = self.state.lock().unwrap();

                state.query_page += 1;
                state.bookmark_cards.len()
            };

            self.fetch_and_splice(position);
        }

        pub fn fetch_and_splice(&self, position: usize) {
            let (bookmarks, count, query, page, page_size) = {
                let state = self.state.lock().unwrap();

                (
                    state.bookmarks.clone(),
                    state.bookmark_cards.len(),
                    state.query.clone(),
                    state.query_page,
                    state.settings
                        .get()
                        .map(|s| s.get::<u32>("max-bookmarks-per-query") as i32)
                        .unwrap_or(1000),
                )
            };

            // A copy for the callback, to avoid premature destruction
            let store = self.obj().clone();

            glib::spawn_future_local(async move {
                let reply = gio::spawn_blocking(move || { bookmarks.search(query, page, page_size) }).await;

                let Ok(matches) = reply else {
                    return
                };

                if matches.len() < 1 {
                    return;
                }

                if count < position as usize {
                    return;
                }

                let added = matches.len();
                let taken = {
                    let mut state = store.imp().state.lock().unwrap();

                    let cards = matches
                        .iter()
                        .map(BookmarkCard::new)
                        .collect::<Vec<BookmarkCard>>();

                    state
                        .bookmark_cards
                        .splice(position..count, cards)
                        .count()
                };

                store.items_changed(position as u32, taken as u32, added as u32);
            });
        }
    }

    impl ObjectImpl for BookmarkPagination {}
}

glib::wrapper! {
    pub struct BookmarkPagination(ObjectSubclass<imp::BookmarkPagination>)
        @implements gio::ListModel;
}

impl BookmarkPagination {
    pub fn new(api: BookmarkStorage, cfg: gio::Settings) -> Self {
        let object: BookmarkPagination = glib::Object::builder().build();

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

impl Default for BookmarkPagination {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}

