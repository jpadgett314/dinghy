use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use gtk::{glib};
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct EpochCounter {
        pub latest: Arc<AtomicU64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EpochCounter {
        const NAME: &'static str = "MyEpochCounter";
        type Type = super::EpochCounter;
    }

    impl ObjectImpl for EpochCounter {}
}

glib::wrapper! {
    pub struct EpochCounter(ObjectSubclass<imp::EpochCounter>);
}

impl EpochCounter {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn create(&self) -> Epoch {
        let latest = Arc::clone(&self.imp().latest);
        let token = latest.fetch_add(1, Ordering::Relaxed) + 1;

        Epoch { latest, token }
    }
}

pub struct Epoch {
    latest: Arc<AtomicU64>,
    token: u64,
}

impl Epoch {
    pub fn is_latest(&self) -> bool {
        self.latest.load(Ordering::Relaxed) == self.token
    }

    pub fn value(&self) -> u64 {
        self.token
    }
}

