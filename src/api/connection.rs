use std::sync::{Arc, RwLock};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
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
        #[property(get, set = Self::set_url, nullable)]
        pub url: Arc<RwLock<Option<String>>>,
        #[property(get, set = Self::set_token, nullable)]
        pub token: Arc<RwLock<Option<String>>>,
    }

    impl Connection {
        pub fn set_api(&self, url: &str, token: &str) {
            *self.api.write().unwrap() = Some(LinkDingClient::new(url, token));
        }

        pub fn set_url(&self, value: Option<String>) {
            if let Some(token) = self.token.read().unwrap().as_ref() {
                if let Some(url) = value.as_ref() {
                    self.set_api(url, token);
                }
            }

            *self.url.write().unwrap() = value.clone();
        }

        pub fn set_token(&self, value: Option<String>) {
            if let Some(url) = self.url.read().unwrap().as_ref() {
                if let Some(token) = value.as_ref() {
                    self.set_api(url, token);
                }
            }

            *self.token.write().unwrap() = value.clone();
        }
    }

    impl Connection {

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
    pub fn new(url: &String, token: &String) -> Self {
        glib::Object::builder()
            .property("url", url)
            .property("token", token)
            .build()
    }

    pub fn from(settings: &gio::Settings) -> Self {
        let object: Self = glib::Object::builder()
            .property("url", settings.get::<String>("api-url"))
            .property("token", settings.get::<String>("api-token"))
            .build();

        object.bind(settings);
        object
    }

    pub fn bind(&self, settings: &gio::Settings) {
        settings
            .bind("api-url", self, "url")
            .build();
        settings
            .bind("api-token", self, "token")
            .build();
    }

    pub fn use_api<R>(&self, operation: impl FnOnce(&LinkDingClient) -> R) -> Option<R> {
        // `LinkDingClient` is a blocking synchronous API!!
        assert!(
            !gtk::is_initialized_main_thread(),
            "Connection::use_api() must not run on the GTK main thread."
        );

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

