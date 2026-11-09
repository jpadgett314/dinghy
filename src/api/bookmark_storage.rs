use std::sync::RwLock;

use gtk::glib;
use gtk::glib::prelude::*;
use gtk::glib::subclass::prelude::*;
use linkding::ListBookmarksArgs;

use crate::api::{to_create_req, to_internal, to_update_req, Bookmark, BookmarkChanges, BookmarkToAdd, Connection};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::BookmarkStorage)]
    pub struct BookmarkStorage {
        #[property(get, set, nullable)]
        pub connection: RwLock<Option<Connection>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkStorage {
        const NAME: &'static str = "BookmarkStorage";
        type Type = super::BookmarkStorage;
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkStorage {}
}

glib::wrapper! {
    pub struct BookmarkStorage(ObjectSubclass<imp::BookmarkStorage>);
}

impl BookmarkStorage {
    pub fn new(connection: &Connection) -> Self {
        glib::Object::builder()
            .property("connection", connection)
            .build()
    }

    pub fn search(&self, q: String, p: i32, ps: i32) -> Vec<Bookmark> {
        let Some(connection) = self.connection() else {
            return vec![];
        };

        let req = ListBookmarksArgs {
            query: Some(q),
            limit: Some(ps),
            offset: Some(ps * p),
            modified_since: None,
        };

        let res = connection.use_api(|api| api.list_bookmarks(req));

        match res {
            Ok(data) => data.results.iter().map(to_internal).collect(),
            Err(_) => vec![],
        }
    }

    pub fn create(&self, b: &BookmarkToAdd) -> Option<Bookmark> {
        let req = to_create_req(b);

        let res = self.connection()?.use_api(|api| api.create_bookmark(req));

        match res {
            Ok(data) => Some(to_internal(&data)),
            Err(_) => None,
        }
    }

    pub fn update(&self, b: &BookmarkChanges) -> Option<Bookmark> {
        let req = to_update_req(b);

        let res = self.connection()?.use_api(|api| api.update_bookmark(b.id, req));

        match res {
            Ok(data) => Some(to_internal(&data)),
            Err(_) => None,
        }
    }

    pub fn delete(&self, id: i32) -> bool {
        let Some(connection) = self.connection() else {
            return false;
        };

        let res = connection.use_api(|api| api.delete_bookmark(id));

        match res {
            Ok(data) => data,
            Err(_) => false,
        }
    }
}

impl Default for BookmarkStorage {
    fn default() -> Self {
        glib::Object::new()
    }
}

