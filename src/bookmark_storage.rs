use std::sync::Arc;

use gtk::glib;
use linkding::{LinkDingClient, ListBookmarksArgs};

use crate::bookmark_record::{BookmarkRecord, to_create_req, to_internal, to_update_req};

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "DinghyBookmarkStorage", nullable)]
pub struct BookmarkStorage {
    pub api: Arc<LinkDingClient>,
}

impl BookmarkStorage {
    pub fn new(api: LinkDingClient) -> Self {
        Self {
            api: Arc::new(api),
        }
    }

    pub fn search(&self, q: String, p: i32, s: i32) -> Vec<BookmarkRecord> {
        let req = ListBookmarksArgs {
            query: Some(q),
            limit: Some(s),
            offset: Some(s * p),
            modified_since: None,
        };

        let res = self.api.list_bookmarks(req);

        match res {
            Ok(data) => {
                data.results
                    .iter()
                    .map(to_internal)
                    .collect()
            }
            Err(e) => {
                eprintln!("API error: {e}");
                vec![]
            }
        }
    }

    pub fn create(&self, b: &BookmarkRecord) -> BookmarkRecord {
        let req = to_create_req(b);

        let res = self.api.create_bookmark(req);

        match res {
            Ok(data) => to_internal(&data),
            Err(e) => {
                eprintln!("API error: {e}");
                BookmarkRecord::default()
            }
        }
    }

    pub fn update(&self, b: &BookmarkRecord) -> BookmarkRecord {
        let req = to_update_req(b);

        let res = self.api.update_bookmark(b.id, req);

        match res {
            Ok(data) => to_internal(&data),
            Err(e) => {
                eprintln!("API error: {e}");
                BookmarkRecord::default()
            }
        }
    }

    pub fn delete(&self, id: i32) -> bool {
        let res = self.api.delete_bookmark(id);

        match res {
            Ok(data) => data,
            Err(e) => {
                eprintln!("API error: {e}");
                false
            }
        }
    }
}

impl Default for BookmarkStorage {
    fn default() -> Self {
        Self {
            api: Arc::new(LinkDingClient::new("localhost:9090", "")),
        }
    }
}
