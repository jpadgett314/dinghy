use adw::prelude::*;
use adw::subclass::prelude::*;
use gio::ListStore;
use gtk::{gio, glib};

use crate::bookmark_record::BookmarkRecord;
use crate::widgets::BookmarkCard;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/widgets/bookmark_feed.ui")]
    pub struct BookmarkFeed {
        pub model: std::cell::OnceCell<ListStore>,
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListView>,
        #[template_child]
        pub selection: gtk::TemplateChild<gtk::NoSelection>,
        #[template_child]
        pub factory: gtk::TemplateChild<gtk::SignalListItemFactory>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkFeed {
        const NAME: &'static str = "BookmarkFeed";
        type Type = super::BookmarkFeed;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookmarkFeed {
        fn constructed(&self) {
            self.parent_constructed();

            let model = ListStore::new::<BookmarkCard>();

            self.factory.connect_setup(|_, item| {
                let item = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem");
                item.set_selectable(false);
                item.set_activatable(true);
            });

            self.factory.connect_bind(|_, item| {
                let item = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem");
                let row = item
                    .item()
                    .and_downcast::<BookmarkCard>()
                    .expect("model item should be a BookmarkCard");
                item.set_child(Some(&row));
            });

            self.factory.connect_unbind(|_, item| {
                item.downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem")
                    .set_child(None::<&gtk::Widget>);
            });

            self.selection.set_model(Some(&model));

            let weak_list = self.obj().downgrade();
            self.list.connect_activate(move |_, position| {
                if let Some(list) = weak_list.upgrade() {
                    if let Some(row) = list.model().item(position).and_downcast::<BookmarkCard>() {
                        row.open();
                    }
                }
            });

            self.model
                .set(model)
                .expect("model should only be set once");
        }
    }

    impl WidgetImpl for BookmarkFeed {}
    impl BoxImpl for BookmarkFeed {}
}

glib::wrapper! {
    pub struct BookmarkFeed(ObjectSubclass<imp::BookmarkFeed>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BookmarkFeed {
    pub fn replace(&self, bookmarks: &Vec<BookmarkRecord>) {
        let rows: Vec<BookmarkCard> = bookmarks.iter().map(BookmarkCard::new).collect();
        self.model().splice(0, self.model().n_items(), &rows);
    }

    pub fn push(&self, bookmarks: &Vec<BookmarkRecord>) {
        let rows: Vec<BookmarkCard> = bookmarks.iter().map(BookmarkCard::new).collect();
        self.model().splice(self.model().n_items(), 0, &rows);
    }

    fn model(&self) -> &ListStore {
        self.imp()
            .model
            .get()
            .expect("BookmarkFeed model should be initialized")
    }
}

