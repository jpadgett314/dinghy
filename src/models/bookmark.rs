use linkding::{LinkDingClient, ListBookmarksArgs};

use crate::bookmark_record::{BookmarkRecord, to_create_req, to_internal, to_update_req};

#[derive(Debug)]
pub struct BookmarkModel {
    pub api: LinkDingClient
}

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

impl Default for BookmarkModel {
    fn default() -> Self {
        Self { api: LinkDingClient::new("localhost:9090", "") }
    }
}

