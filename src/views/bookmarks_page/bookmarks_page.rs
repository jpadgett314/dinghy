use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::{BookmarkStorage, Connection};
use crate::models::{BookmarkFeedModel, ToolbarModel};
use crate::util::{action_entry0, action_entry1};
use crate::views::{AddBookmarkDialog, BookmarkFeed, EditBookmarkDialog};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::BookmarksPage)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmarks_page/bookmarks_page.ui")]
    pub struct BookmarksPage {
        #[property(get)]
        pub actions: gio::SimpleActionGroup,
        #[property(get, set, nullable)]
        pub connection: RefCell<Option<Connection>>,
        #[property(get, set, nullable)]
        pub settings: RefCell<Option<gio::Settings>>,
        #[property(get, set)]
        pub toolbar_model: RefCell<ToolbarModel>,
        #[property(get, nullable)]
        pub bookmark_storage: RefCell<Option<BookmarkStorage>>,
        #[property(get, nullable)]
        pub bookmark_feed_model: RefCell<Option<BookmarkFeedModel>>,
        #[template_child]
        pub bookmark_feed_widget: TemplateChild<BookmarkFeed>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarksPage {
        const NAME: &'static str = "BookmarksPage";
        type Type = super::BookmarksPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            ToolbarModel::ensure_type();
            BookmarkFeed::ensure_type();
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarksPage {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_gactions();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for BookmarksPage {}
    impl BoxImpl for BookmarksPage {}
}

glib::wrapper! {
    pub struct BookmarksPage(ObjectSubclass<imp::BookmarksPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl BookmarksPage {
    fn setup_gactions(&self) {
        let entries = [
            action_entry0(
                self,
                "open-add-dialog",
                Self::on_open_add_dialog_action,
            ),
            action_entry1(
                self,
                "search",
                Some(glib::VariantTy::STRING),
                Self::on_search_bookmarks_action,
            ),
            action_entry1(
                self,
                "open-edit-dialog",
                Some(glib::VariantTy::INT32),
                Self::on_open_edit_dialog_action,
            ),
        ];
        self.actions().add_action_entries(entries);
    }

    fn on_open_add_dialog_action(&self) {
        let Some(storage) = self.bookmark_storage() else {
            return;
        };
        AddBookmarkDialog::new(&storage).present(Some(self));
    }

    fn on_open_edit_dialog_action(&self, param: Option<&glib::Variant>) {
        let id = param.unwrap().get::<i32>().unwrap();
        let Some(feed) = self.bookmark_feed_model() else {
            return;
        };
        let Some(storage) = self.bookmark_storage() else {
            return;
        };
        let Some(model) = feed.find_by_id(id) else {
            return;
        };
        EditBookmarkDialog::new(&storage, &model).present(Some(self));
    }

    fn on_search_bookmarks_action(&self, param: Option<&glib::Variant>) {
        let query = param.unwrap().get::<String>().unwrap();
        self.toolbar_model().set_searchbar_open(true);
        self.toolbar_model().set_query(query);
    }

    #[template_callback]
    fn on_dependencies_changed(&self) {
        if self.bookmark_feed_model().is_some() {
            return;
        }
        let (Some(connection), Some(settings)) = (self.connection(), self.settings()) else {
            return;
        };
        let storage = BookmarkStorage::new(&connection);
        let feed = BookmarkFeedModel::new(storage.clone(), settings);
        self.imp().bookmark_storage.replace(Some(storage));
        self.imp().bookmark_feed_model.replace(Some(feed.clone()));
        self.imp().bookmark_feed_widget.set_model(feed);
        self.refresh();
    }

    #[template_callback]
    fn on_query_changed(&self) {
        self.refresh();
    }

    pub fn refresh(&self) {
        if let Some(feed) = self.bookmark_feed_model() {
            feed.fetch_query(self.toolbar_model().query());
        }
    }
}
