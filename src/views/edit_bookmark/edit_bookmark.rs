use std::cell::OnceCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};

use crate::api::{BookmarkStorage, BookmarkChanges};
use crate::models::BookmarkCardModel;
use crate::widgets::TagEntryRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::EditBookmarkDialog)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/edit_bookmark/edit_bookmark.ui")]
    pub struct EditBookmarkDialog {
        #[property(get, set, construct_only)]
        pub bookmark_storage: OnceCell<BookmarkStorage>,
        #[property(get, set, construct_only)]
        pub model: OnceCell<BookmarkCardModel>,
        #[template_child]
        pub url_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub name_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub description_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub tags_row: TemplateChild<TagEntryRow>,
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EditBookmarkDialog {
        const NAME: &'static str = "EditBookmarkDialog";
        type Type = super::EditBookmarkDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            TagEntryRow::ensure_type();
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for EditBookmarkDialog {
        fn constructed(&self) {
            self.parent_constructed();

            let model = self.obj().model();
            self.url_row.set_text(&model.url());
            self.name_row.set_text(&model.title());
            self.description_row.set_text(&model.description());
            self.tags_row.set_tag_names(model.tag_names());
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WidgetImpl for EditBookmarkDialog {}
    impl AdwDialogImpl for EditBookmarkDialog {}
}

glib::wrapper! {
    pub struct EditBookmarkDialog(ObjectSubclass<imp::EditBookmarkDialog>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl EditBookmarkDialog {
    pub fn new(storage: &BookmarkStorage, model: &BookmarkCardModel) -> Self {
        glib::Object::builder()
            .property("bookmark-storage", storage)
            .property("model", model)
            .build()
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        self.close();
    }

    #[template_callback]
    fn on_save_clicked(&self) {
        let imp = self.imp();
        imp.tags_row.commit_pending();

        let changes = BookmarkChanges {
            id: imp.model.get().unwrap().id(),
            url: Some(imp.url_row.text().to_string()),
            title: Some(imp.name_row.text().to_string()),
            description: Some(imp.description_row.text().to_string()),
            tag_names: Some(imp.tags_row.tag_names().iter().map(|tag| tag.to_string()).collect()),
        };

        imp.save_button.set_sensitive(false);

        let api = self.bookmark_storage();
        let obj = self.clone();

        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(move || api.update(&changes)).await;
            let success = matches!(result, Ok(Some(_)));
            let message = if success {
                gettext("Bookmark saved")
            } else {
                gettext("Failed to save bookmark")
            };
            let _ = obj.activate_action("win.toast", Some(&message.to_variant()));
            let _ = obj.activate_action("client.refresh", None);
            obj.imp().save_button.set_sensitive(true);
            if success {
                obj.close();
            }
        });
    }
}
