use gtk::glib;
use linkding::ListBookmarksArgs;

use crate::api::{to_create_req, to_internal, to_update_req, Bookmark, BookmarkChanges, BookmarkToAdd, Connection};

#[derive(Debug, Default, Clone, glib::Boxed)]
#[boxed_type(name = "DinghyBookmarkStorage", nullable)]
pub struct BookmarkStorage {
    api: Connection,
}

impl BookmarkStorage {
    pub fn new(connection: &Connection) -> Self {
        Self {
            api: connection.clone(),
        }
    }

    pub fn search(&self, q: String, p: i32, ps: i32) -> Vec<Bookmark> {
        let req = ListBookmarksArgs {
            query: Some(q),
            limit: Some(ps),
            offset: Some(ps * p),
            modified_since: None,
        };

        let res = self.api.use_api(|api| api.list_bookmarks(req));

        match res {
            Ok(data) => data.results.iter().map(to_internal).collect(),
            Err(_) => vec![],
        }
    }

    pub fn create(&self, b: &BookmarkToAdd) -> Option<Bookmark> {
        let req = to_create_req(b);

        let res = self.api.use_api(|api| api.create_bookmark(req));

        match res {
            Ok(data) => Some(to_internal(&data)),
            Err(_) => None,
        }
    }

    pub fn update(&self, b: &BookmarkChanges) -> Option<Bookmark> {
        let req = to_update_req(b);

        let res = self.api.use_api(|api| api.update_bookmark(b.id, req));

        match res {
            Ok(data) => Some(to_internal(&data)),
            Err(_) => None,
        }
    }

    pub fn delete(&self, id: i32) -> bool {
        let res = self.api.use_api(|api| api.delete_bookmark(id));

        match res {
            Ok(data) => data,
            Err(_) => false,
        }
    }
}
