use linkding::{LinkDingClient}; // , ListBookmarksResponse};

pub struct BookmarkData {
    icon_url: str,
    name: str,
    description: str,
}

mod imp {
    pub struct BookmarkModel {
        linkding_api: LinkDingClient
    }

    impl BookmarkModel {

    }
}

// purpose: simply API (pagination mainly)
impl BookmarkModel {
    pub fn new(api: LinkDingClient) -> Self {
        Self { api }
    }

    pub fn search(&self, q: &str, p: i32) -> Vec<BookmarkRecord> {
        {}
    }

    // TODO: submit, delete
}

