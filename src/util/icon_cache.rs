use gtk::{gdk, glib};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

pub struct IconCache {
    agent: ureq::Agent,
    cache: Mutex<HashMap<String, gdk::Texture>>,
}

impl IconCache {
    fn new() -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(10)))
            .build()
            .into();

        Self {
            agent,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<IconCache> = OnceLock::new();
        INSTANCE.get_or_init(IconCache::new)
    }

    pub fn fetch(&self, url: &str) -> Option<gdk::Texture> {
        {
            let cache = self.cache.lock().unwrap();
            if let Some(texture) = cache.get(url) {
                return Some(texture.clone());
            }
        }

        let fetch_bytes = || {
            self.agent
                .get(url)
                .call()?
                .body_mut()
                .with_config()
                .limit(256 * 256) // 64 KB limit
                .read_to_vec()
        };

        let bytes = fetch_bytes().ok()?;
        let texture = gdk::Texture::from_bytes(&glib::Bytes::from_owned(bytes)).ok()?;

        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(url.to_string(), texture.clone());
        }

        Some(texture)
    }
}

pub fn fetch_icon(url: &str) -> Option<gdk::Texture> {
    IconCache::global().fetch(url)
}

