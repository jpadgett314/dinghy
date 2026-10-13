use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, ListView, NoSelection, SignalListItemFactory};
use gio::ListStore;

use crate::bookmark_data::BookmarkData;
use crate::bookmark_row::BookmarkRow;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct BookmarkList {
        pub model: std::cell::OnceCell<ListStore>,
        pub list_view: std::cell::OnceCell<ListView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkList {
        const NAME: &'static str = "BookmarkList";
        type Type = super::BookmarkList;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for BookmarkList {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            let model = ListStore::new::<BookmarkRow>();
            let selection = NoSelection::new(Some(model.clone()));

            let factory = SignalListItemFactory::new();
            factory.connect_bind(|_, item| {
                let item = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem");

                if let Some(row) = item.item().and_downcast::<BookmarkRow>() {
                    item.set_child(Some(&row));
                }
            });

            let list_view = ListView::new(Some(selection), Some(factory));
            list_view.set_vexpand(true);
            list_view.set_hexpand(true);

            obj.append(&list_view);

            self.model.set(model).expect("model should only be set once");
            self.list_view.set(list_view).expect("list_view should only be set once");
        }
    }

    impl WidgetImpl for BookmarkList {}
    impl BoxImpl for BookmarkList {}
}

glib::wrapper! {
    pub struct BookmarkList(ObjectSubclass<imp::BookmarkList>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BookmarkList {
    pub fn replace(&self, bookmarks: &[BookmarkData]) {
        let rows: Vec<BookmarkRow> = bookmarks.iter().map(BookmarkRow::new).collect();
        self.model().splice(0, self.model().n_items(), &rows);
    }

    pub fn push(&self, bookmarks: &[BookmarkData]) {
        let rows: Vec<BookmarkRow> = bookmarks.iter().map(BookmarkRow::new).collect();
        self.model().splice(self.model().n_items(), 0, &rows);
    }

    fn model(&self) -> &ListStore {
        self.imp()
            .model
            .get()
            .expect("BookmarkList model should be initialized")
    }
}

