use std::cell::{Cell, RefCell};
use std::future::Future;

use gettextrs::gettext;
use gtk::glib;
use gtk::glib::subclass::prelude::*;
use gtk::{gio, prelude::*};

use crate::api::BookmarkStorage;
use crate::models::{BatchUpdate, BookmarkCardModel, BookmarkFeedModel};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::BatchOperationModel)]
    pub struct BatchOperationModel {
        pub cancellable: gio::Cancellable,
        #[property(get)]
        pub active: Cell<bool>,
        #[property(get)]
        pub cancelled: Cell<bool>,
        #[property(get)]
        pub completed: Cell<u32>,
        #[property(get)]
        pub fraction: Cell<f64>,
        #[property(get)]
        pub progress: RefCell<String>,
        #[property(get)]
        pub status: RefCell<String>,
        #[property(get, construct_only)]
        pub total: Cell<u32>,
    }

    impl BatchOperationModel {
        pub fn cancel(&self) {
            if !self.active.get() || self.cancelled.get() {
                return;
            }

            self.cancellable.cancel();
            self.cancelled.set(true);
            self.status.replace(gettext("Cancelling..."));

            self.obj().notify_cancelled();
            self.obj().notify_status();
        }

        pub fn complete_one(&self) {
            let completed = self.completed.get() + 1;
            let total = self.total.get();

            self.completed.set(completed);
            self.fraction.set(completed as f64 / total as f64);
            self.progress.replace(format!("{completed} / {total}"));

            self.obj().notify_completed();
            self.obj().notify_fraction();
            self.obj().notify_progress();
        }

        pub fn finish(&self) {
            self.active.set(false);
            self.obj().notify_active();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BatchOperationModel {
        const NAME: &str = "BatchOperationModel";
        type Type = super::BatchOperationModel;
    }

    #[glib::derived_properties]
    impl ObjectImpl for BatchOperationModel {}
}

glib::wrapper! {
    pub struct BatchOperationModel(ObjectSubclass<imp::BatchOperationModel>);
}

impl BatchOperationModel {
    pub fn delete_bookmarks(
        storage: BookmarkStorage,
        feed: BookmarkFeedModel,
        bookmark_ids: Vec<i32>,
    ) -> Self {
        Self::run(gettext("Deleting bookmarks..."), bookmark_ids, move |id| {
            let storage = storage.clone();
            let feed = feed.clone();
            async move {
                let deleted = gio::spawn_blocking(move || storage.delete(id))
                    .await
                    .unwrap_or(false);

                if deleted {
                    feed.remove_by_id(id);
                }
            }
        })
    }

    pub fn edit_bookmarks(
        storage: BookmarkStorage,
        bookmarks: Vec<BookmarkCardModel>,
        edits: BatchUpdate,
    ) -> Option<Self> {
        let updates: Vec<_> = bookmarks
            .into_iter()
            .filter_map(|bookmark| {
                let changes = edits.for_bookmark(&bookmark)?;
                Some((bookmark, changes))
            })
            .collect();

        if updates.is_empty() {
            return None;
        }

        Some(Self::run(
            gettext("Updating bookmarks..."),
            updates,
            move |(bookmark, changes)| {
                let storage = storage.clone();
                async move {
                    if let Ok(Some(updated)) =
                        gio::spawn_blocking(move || storage.update(&changes)).await
                    {
                        bookmark.set_title(updated.title);
                        bookmark.set_description(updated.description);
                        bookmark.set_tag_names(glib::StrV::from(updated.tag_names));
                    }
                }
            },
        ))
    }

    fn run<T: 'static, F: Future<Output = ()> + 'static>(
        status: String,
        items: Vec<T>,
        mut apply: impl FnMut(T) -> F + 'static,
    ) -> Self {
        let total = items.len();
        let operation: Self = glib::Object::builder()
            .property("total", total as u32)
            .build();

        operation.imp().active.set(true);
        operation.imp().status.replace(status);
        operation.imp().progress.replace(format!("0 / {total}"));

        let cancellable = operation.imp().cancellable.clone();
        let operation_clone = operation.clone();

        glib::spawn_future_local(async move {
            for item in items {
                if cancellable.is_cancelled() {
                    break;
                }

                apply(item).await;
                operation_clone.imp().complete_one();
            }

            operation_clone.imp().finish();
        });

        operation
    }

    pub fn cancel(&self) {
        self.imp().cancel();
    }
}
