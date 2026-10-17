use linkding::{LinkDingClient, ListBookmarksArgs};

#[derive(Debug, Default, Copy, Clone)]
pub struct CollectionStats {
    pub bookmark_count: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Debug)]
pub struct Connection {
    api: LinkDingClient,
}

impl Connection {
    pub fn new(api: LinkDingClient) -> Self {
        Self { api }
    }

    pub fn status(&self) -> ConnectionStatus {
        let req = ListBookmarksArgs {
            query: Some(String::from("")),
            limit: Some(0),
            offset: Some(0),
            modified_since: None,
        };

        let res = self.api.list_bookmarks(req);

        match res {
            Ok(_) => ConnectionStatus::Connected,
            Err(_) => ConnectionStatus::Disconnected,
        }
    }

    pub fn stats(&self) -> Option<CollectionStats> {
        let req = ListBookmarksArgs {
            query: Some(String::from("")),
            limit: Some(0),
            offset: Some(0),
            modified_since: None,
        };

        let res = self.api.list_bookmarks(req);

        match res {
            Ok(data) => {
                let stats = CollectionStats { bookmark_count: data.count };
                Some(stats)
            }
            Err(e) => {
                eprintln!("API error: {e}");
                None
            }
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self { api: LinkDingClient::new("localhost:9090", "") }
    }
}

