use std::cell::{OnceCell, RefCell};

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
        #[property(get, set)]
        pub bookmark_storage: OnceCell<BookmarkStorage>,
        #[property(get, set)]
        pub bookmark_feed_model: OnceCell<BookmarkFeedModel>,
        #[template_child]
        pub status_stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarksPage {
        const NAME: &'static str = "BookmarksPage";
        type Type = super::BookmarksPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            ToolbarModel::ensure_type();
            BookmarkStorage::ensure_type();
            BookmarkFeedModel::ensure_type();
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
                "open-edit-dialog",
                Some(glib::VariantTy::INT32),
                Self::on_open_edit_dialog_action,
            ),
            action_entry1(
                self,
                "search",
                Some(glib::VariantTy::STRING),
                Self::on_search_bookmarks_action,
            ),
        ];

        self.actions().add_action_entries(entries);
    }

    fn on_open_add_dialog_action(&self) {
        let storage = self.bookmark_storage();

        AddBookmarkDialog::new(&storage).present(Some(self));
    }

    fn on_open_edit_dialog_action(&self, param: Option<&glib::Variant>) {
        let id = param.unwrap().get::<i32>().unwrap();
        let feed = self.bookmark_feed_model();
        let storage = self.bookmark_storage();
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
    fn on_pending_changed(&self) {
        let model = self.bookmark_feed_model();

        let name = if model.pending() {
            "loading"
        } else if model.n_items() > 0 {
            "bookmarks"
        } else if model.query() == "" {
            "no-bookmarks"
        } else {
            "no-matches"
        };

        self.imp().status_stack.set_visible_child_name(name);
    }

    #[template_callback]
    fn on_query_changed(&self) {
        self.refresh();
    }

    #[template_callback]
    fn on_connection_changed(&self) {
        self.refresh();
    }

    pub fn refresh(&self) {
        self.bookmark_feed_model().fetch_query(self.toolbar_model().query());
    }
}
