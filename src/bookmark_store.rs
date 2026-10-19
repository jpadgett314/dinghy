use std::sync::Mutex;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use gio::prelude::ListModelExt;
use gio::spawn_blocking;
use glib::spawn_future_local;
use linkding::LinkDingClient;

use crate::widgets::BookmarkCard;
use crate::models::BookmarkModel;

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct BookmarkStoreState {
        pub bookmarks: BookmarkModel,
        pub bookmark_cards: Vec<BookmarkCard>,
        pub query: String,
        pub query_page: i32,
    }

    #[derive(Debug)]
    pub struct BookmarkStore {
        pub state: Mutex<BookmarkStoreState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkStore {
        const NAME: &str = "BookmarkStore";
        type Type = super::BookmarkStore;
        type ParentType = glib::Object;
        type Interfaces = (gio::ListModel,);
    }

    impl Default for BookmarkStore {
        fn default() -> Self {
            let state = BookmarkStoreState {
                bookmarks: BookmarkModel::default(),
                bookmark_cards: Vec::default(),
                query: String::new(),
                query_page: 0,
            };

            Self { state: Mutex::new(state) }
        }
    }

    impl ListModelImpl for BookmarkStore {
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

    impl BookmarkStore {
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
            let (bookmarks, query, page, count) = {
                let state = self.state.lock().unwrap();

                (
                    state.bookmarks.clone(),
                    state.query.clone(),
                    state.query_page,
                    state.bookmark_cards.len(),
                )
            };

            // A copy for the callback, to avoid premature destruction
            let store = self.obj().clone();

            spawn_future_local(async move {
                let reply = spawn_blocking(move || { bookmarks.search(query, page) }).await;

                // Connection error
                let Ok(matches) = reply else { return };

                // No bookmarks found
                if matches.len() < 1 { return; }

                // Attempting to place beyond end
                if count < position as usize { return; }

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

    impl ObjectImpl for BookmarkStore {}
}

glib::wrapper! {
    pub struct BookmarkStore(ObjectSubclass<imp::BookmarkStore>)
        @implements gio::ListModel;
}

impl BookmarkStore {
    pub fn new(api: LinkDingClient) -> Self {
        let object: BookmarkStore = glib::Object::builder().build();

        {
            object
                .imp()
                .state
                .lock()
                .unwrap()
                .bookmarks = BookmarkModel::new(api);
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

impl Default for BookmarkStore {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}

