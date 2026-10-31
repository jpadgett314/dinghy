use std::cell::OnceCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};

use crate::api::{Bookmark, BookmarkStorage};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::AddBookmarkDialog)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/add_bookmark/add_bookmark.ui")]
    pub struct AddBookmarkDialog {
        #[property(get, set, construct_only)]
        pub bookmark_storage: OnceCell<BookmarkStorage>,
        #[template_child]
        pub url_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub name_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub description_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub tags_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub create_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddBookmarkDialog {
        const NAME: &'static str = "AddBookmarkDialog";
        type Type = super::AddBookmarkDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AddBookmarkDialog {
        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WidgetImpl for AddBookmarkDialog {}
    impl AdwDialogImpl for AddBookmarkDialog {}
}

glib::wrapper! {
    pub struct AddBookmarkDialog(ObjectSubclass<imp::AddBookmarkDialog>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl AddBookmarkDialog {
    pub fn new(storage: &BookmarkStorage) -> Self {
        glib::Object::builder()
            .property("bookmark-storage", storage)
            .build()
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        self.close();
    }

    #[template_callback]
    fn on_create_clicked(&self) {
        let imp = self.imp();

        let bookmark = Bookmark {
            url: imp.url_row.text().to_string(),
            title: imp.name_row.text().to_string(),
            description: imp.description_row.text().to_string(),
            tag_names: imp.tags_row.text().split_whitespace().map(str::to_owned).collect(),
            ..Default::default()
        };

        imp.create_button.set_sensitive(false);

        let api = self.bookmark_storage();
        let obj = self.clone();

        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(move || api.create(&bookmark)).await;
            let success = matches!(result, Ok(Some(_)));
            let message = if success {
                gettext("Bookmark created")
            } else {
                gettext("Failed to create bookmark")
            };
            let _ = obj.activate_action("win.toast", Some(&message.to_variant()));
            let _ = obj.activate_action("client.refresh-bookmarks", None);
            obj.imp().create_button.set_sensitive(true);
            if success {
                obj.close();
            }
        });
    }
}
