use linkding::{LinkDingClient, ListBookmarksArgs};

use crate::bookmark_record::{BookmarkRecord, to_internal};

#[derive(Debug)]
pub struct BookmarkModel {
    pub api: LinkDingClient
}

// purpose: simply API (pagination mainly)
impl BookmarkModel {
    pub fn new(api: LinkDingClient) -> Self {
        Self { api }
    }

    pub fn search(&self, q: String, p: i32) -> Vec<BookmarkRecord> {
        let req = ListBookmarksArgs {
            query: Some(q),
            limit: Some(100),
            offset: Some(100 * p),
            modified_since: None,
        };

        let res = self.api.list_bookmarks(req);

        match res {
            Ok(data) => {
                data.results
                    .iter()
                    .map(|v| to_internal(v))
                    .collect()
            }
            Err(e) => {
                eprintln!("API error: {e}");
                vec![]
            }
        }
    }

    // TODO: create, update, delete
}

impl Default for BookmarkModel {
    fn default() -> Self {
        Self { api: LinkDingClient::new("localhost:9090", "") }
    }
}

