use std::cell::OnceCell;

use gtk::{glib, subclass::prelude::*};
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

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Connection {
        pub api: OnceCell<LinkDingClient>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Connection {
        const NAME: &'static str = "DinghyConnection";
        type Type = super::Connection;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Connection {}
}

glib::wrapper! {
    pub struct Connection(ObjectSubclass<imp::Connection>);
}

impl Connection {
    pub fn new(api: LinkDingClient) -> Self {
        let object: Self = glib::Object::new();
        object.imp().api.set(api).unwrap();
        object
    }

    pub fn api(&self) -> LinkDingClient {
        self.imp()
            .api
            .get()
            .expect("Connection should have an API client")
            .clone()
    }

    pub fn status(&self) -> ConnectionStatus {
        let req = ListBookmarksArgs {
            query: Some(String::from("")),
            limit: Some(0),
            offset: Some(0),
            modified_since: None,
        };

        let res = self.api().list_bookmarks(req);

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

        let res = self.api().list_bookmarks(req);

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
        Self::new(LinkDingClient::new("localhost:9090", ""))
    }
}

