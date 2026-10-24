use std::sync::{Arc, RwLock};

use gtk::glib;
use gtk::subclass::prelude::*;
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

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Connection)]
    pub struct Connection {
        pub api: Arc<RwLock<Option<LinkDingClient>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Connection {
        const NAME: &'static str = "DinghyConnection";
        type Type = super::Connection;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Connection {}
}

glib::wrapper! {
    pub struct Connection(ObjectSubclass<imp::Connection>);
}

impl Connection {
    pub fn new(url: &str, token: &str) -> Self {
        let object: Self = glib::Object::new();
        object.set_api(url, token);
        object
    }

    pub fn set_api(&self, url: &str, token: &str) {
        let api = LinkDingClient::new(url, token);

        *self.imp().api.write().unwrap() = Some(api);
    }

    pub fn use_api<R>(&self, operation: impl FnOnce(&LinkDingClient) -> R) -> Option<R> {
        let api = self.imp().api.read().unwrap().clone()?;

        Some(operation(&api))
    }

    pub fn status(&self) -> ConnectionStatus {
        let req = ListBookmarksArgs {
            query: Some(String::from("")),
            limit: Some(0),
            offset: Some(0),
            modified_since: None,
        };

        let res = self.use_api(|api| api.list_bookmarks(req));

        match res {
            Some(Ok(_)) => ConnectionStatus::Connected,
            _ => ConnectionStatus::Disconnected,
        }
    }

    pub fn stats(&self) -> Option<CollectionStats> {
        let req = ListBookmarksArgs {
            query: Some(String::from("")),
            limit: Some(0),
            offset: Some(0),
            modified_since: None,
        };

        let res = self.use_api(|api| api.list_bookmarks(req))?;

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
        glib::Object::new()
    }
}
