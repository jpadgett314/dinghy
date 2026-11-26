use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::ngettext;
use gtk::{gio, glib};

use crate::api::{BookmarkStorage, Connection};
use crate::models::{BatchOperationModel, BookmarkFeedModel, ToolbarModel};
use crate::util::{action_entry0, action_entry1};
use crate::views::{
    AddBookmarkDialog, BatchEditDialog, BatchToolbar, BookmarkFeed, EditBookmarkDialog,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::BookmarksPage)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmarks_page/bookmarks_page.ui")]
    pub struct BookmarksPage {
        #[property(get)]
        pub actions: gio::SimpleActionGroup,
        #[property(get, set, nullable)]
        pub batch_operation: RefCell<Option<BatchOperationModel>>,
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
        pub bookmark_feed: TemplateChild<BookmarkFeed>,
        #[template_child]
        pub delete_confirmation: TemplateChild<adw::AlertDialog>,
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
            BatchOperationModel::ensure_type();
            BatchToolbar::ensure_type();
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
    #[template_callback(function)]
    fn should_reveal_batch_toolbar(selected_count: u32, operation_active: bool) -> bool {
        selected_count > 0 || operation_active
    }

    fn setup_gactions(&self) {
        let entries = [
            action_entry0(self, "open-add-dialog", Self::on_open_add_dialog_action),
            action_entry0(self, "edit-selected", Self::on_edit_selected_action),
            action_entry0(self, "delete-selected", Self::on_delete_selected_action),
            action_entry0(self, "select-all", Self::on_select_all_action),
            action_entry0(self, "clear-selection", Self::on_clear_selection_action),
            action_entry0(self, "cancel-batch-operation", Self::on_cancel_batch_operation_action),
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

    fn batch_operation_active(&self) -> bool {
        self.batch_operation()
            .is_some_and(|operation| operation.active())
    }

    fn on_edit_selected_action(&self) {
        if self.batch_operation_active() {
            return;
        }

        let bookmarks = self.imp().bookmark_feed.selected_bookmarks();
        if bookmarks.is_empty() {
            return;
        }

        let edit_dialog = BatchEditDialog::new(
            glib::clone!(
                #[weak(rename_to = page)]
                self.clone(),
                move |changes| {
                    if let Some(operation) =
                        BatchOperationModel::edit_bookmarks(page.bookmark_storage(), bookmarks, changes)
                    {
                        page.set_batch_operation(Some(operation));
                    }
                }
            )
        );

        edit_dialog.present(Some(self));
    }

    fn on_delete_selected_action(&self) {
        if self.batch_operation_active() {
            return;
        }

        let count = self.imp().bookmark_feed.selected_count();
        if count == 0 {
            return;
        }

        let heading = ngettext("Delete Bookmark?", "Delete Bookmarks?", count);
        let body = ngettext(
            "This bookmark will be permanently deleted.",
            "These %u bookmarks will be permanently deleted.",
            count,
        )
        .replace("%u", &count.to_string());

        let dialog = self.imp().delete_confirmation.clone();
        dialog.set_heading(Some(&heading));
        dialog.set_body(&body);
        dialog.present(Some(self));
    }

    fn on_select_all_action(&self) {
        if self.imp().bookmark_feed.is_mapped() {
            self.imp().bookmark_feed.select_all();
        }
    }

    fn on_clear_selection_action(&self) {
        self.imp().bookmark_feed.clear_selection();
    }

    fn on_cancel_batch_operation_action(&self) {
        if let Some(operation) = self.batch_operation() {
            operation.cancel();
        }
    }

    #[template_callback]
    fn on_delete_confirmation_response(&self, response: &str) {
        if response != "delete" {
            return;
        }

        let bookmark_ids = self
            .imp()
            .bookmark_feed
            .selected_bookmarks()
            .iter()
            .map(|bookmark| bookmark.id())
            .collect();

        let operation = BatchOperationModel::delete_bookmarks(
            self.bookmark_storage(),
            self.bookmark_feed_model(),
            bookmark_ids,
        );

        self.set_batch_operation(Some(operation));
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
