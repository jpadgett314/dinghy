use std::sync::{Arc, RwLock};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use linkding::{LinkDingClient, LinkDingError, ListBookmarksArgs};

use crate::api::ApiError;
use crate::util::EpochCounter;

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
        #[property(get, set)]
        pub epoch_counter: Arc<RwLock<Option<EpochCounter>>>,
        #[property(get, set)]
        pub lost: Arc<RwLock<bool>>,
    }

    impl Connection {
        pub fn set_api(&self, url: &str, token: &str) {
            *self.api.write().unwrap() = Some(LinkDingClient::new(url, token));
            // The caller should do it if desired
            // self.obj().poll();
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

        pub fn poll_from_worker(&self) {
            let req = ListBookmarksArgs {
                query: Some(String::from("POLL")),
                limit: Some(0),
                offset: Some(0),
                modified_since: None,
            };

            let _ = self.obj().use_api(|api| api.list_bookmarks(req));
        }

        pub fn poll_from_main(&self) {
            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = connection)]
                self,
                async move {
                    let _ = gio::spawn_blocking(move || connection.poll_from_worker()).await;
                }
            ));
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Connection {
        const NAME: &'static str = "DinghyConnection";
        type Type = super::Connection;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Connection {
        fn constructed(&self) {
            self.parent_constructed();
            *self.epoch_counter.write().unwrap() = Some(EpochCounter::new());
        }
    }
}

glib::wrapper! {
    pub struct Connection(ObjectSubclass<imp::Connection>);
}

impl Connection {
    pub fn new(url: &String, token: &String) -> Self {
        glib::Object::builder()
            .property("url", url)
            .property("token", token)
            .property("lost", false)
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
        if gtk::is_initialized_main_thread() {
            self.imp().poll_from_main();
        } else {
            self.imp().poll_from_worker();
        }
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

        let epoch = self
            .epoch_counter()
            .expect("epoch_counter should be initialized.")
            .create();

        let ret = operation(&api);

        let err = ret.as_ref().map_err(|e| {
            eprintln!("API error: {e}");
            ApiError::from(e)
        });

        let new_status = match err {
            Err(ApiError::Unreachable) => true,
            Err(_) => false,
            Ok(_) => false,
        };

        glib::idle_add_once(glib::clone!(
            #[weak(rename_to = connection)]
            self.clone(),
            move || {
                if epoch.is_latest() {
                    connection.set_lost(new_status);
                }
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
            .property("lost", false)
            .build();

        object.bind(settings);
        object.poll();
        object
    }
}

impl Default for Connection {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
