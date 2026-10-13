use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::ngettext;
use gio::ListStore;
use gtk::{gio, glib};
use linkding::Bookmark;

use crate::widgets::BookmarkRow;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/widgets/bookmark_list.ui")]
    pub struct BookmarkList {
        pub model: std::cell::OnceCell<ListStore>,
        #[template_child]
        pub count: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkList {
        const NAME: &'static str = "BookmarkList";
        type Type = super::BookmarkList;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookmarkList {
        fn constructed(&self) {
            self.parent_constructed();

            let model = ListStore::new::<BookmarkRow>();
            self.list.bind_model(Some(&model), |item| {
                item.downcast_ref::<BookmarkRow>()
                    .expect("model item should be a BookmarkRow")
                    .clone()
                    .upcast()
            });

            let count = self.count.get();
            model.connect_items_changed(move |model, _, _, _| {
                let total = model.n_items();
                count.set_label(
                    &ngettext("{} bookmark", "{} bookmarks", total)
                        .replace("{}", &total.to_string()),
                );
                count.set_visible(total > 0);
            });

            self.model
                .set(model)
                .expect("model should only be set once");
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
    pub fn replace(&self, bookmarks: &Vec<Bookmark>) {
        let rows: Vec<BookmarkRow> = bookmarks.iter().map(BookmarkRow::new).collect();
        self.model().splice(0, self.model().n_items(), &rows);
    }

    pub fn push(&self, bookmarks: &Vec<Bookmark>) {
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
