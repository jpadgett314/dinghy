use std::sync::{Arc, RwLock};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use linkding::{LinkDingClient, LinkDingError, ListBookmarksArgs};

use crate::api::ApiError;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Connection)]
    pub struct Connection {
        pub api: Arc<RwLock<Option<LinkDingClient>>>,
        #[property(get, set)]
        pub available: Arc<RwLock<bool>>,
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

    pub fn bind(&self, settings: &gio::Settings) {
        settings
            .bind("api-url", self, "url")
            .build();
        settings
            .bind("api-token", self, "token")
            .build();
    }

    pub fn poll(&self) {
        let req = ListBookmarksArgs {
            query: Some(String::from("POLL")),
            limit: Some(0),
            offset: Some(0),
            modified_since: None,
        };

        let _ = self.use_api(|api| api.list_bookmarks(req));
    }

    pub fn use_api<R>(&self, operation: impl FnOnce(&LinkDingClient) -> Result<R, LinkDingError>) -> Result<R, LinkDingError> {
        // `LinkDingClient` is a blocking synchronous API!!
        assert!(
            !gtk::is_initialized_main_thread(),
            "Connection::use_api() must not run on the GTK main thread."
        );

        let api = self
            .imp()
            .api
            .read()
            .unwrap()
            .clone()
            .expect("LinkDingClient should be initialized.");

        let ret = operation(&api);

        let err = ret.as_ref().map_err(|e| {
            eprintln!("API error: {e}");
            ApiError::from(e)
        });

        let new_status = match err {
            Err(ApiError::RequestFailed) => true,
            Err(_) => false,
            Ok(_) => true,
        };

        glib::idle_add_once(glib::clone!(
            #[weak(rename_to = connection)]
            self.clone(),
            move || {
                connection.set_available(new_status);
            }
        ));

        ret
    }
}

impl From<&gio::Settings> for Connection {
    fn from(settings: &gio::Settings) -> Self {
        let object: Self = glib::Object::builder()
            .property("url", settings.get::<String>("api-url"))
            .property("token", settings.get::<String>("api-token"))
            .build();

        object.bind(settings);
        object.poll();
        object
    }
}

impl Default for Connection {
    fn default() -> Self {
        glib::Object::new()
    }
}
