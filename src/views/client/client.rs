use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::Connection;
use crate::models::ToolbarModel;
use crate::views::BookmarksPage;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::DinghyClient)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/client/client.ui")]
    pub struct DinghyClient {
        #[property(get, set, nullable)]
        pub connection: RefCell<Option<Connection>>,
        #[property(get, set, nullable)]
        pub settings: RefCell<Option<gio::Settings>>,
        #[property(get, set, nullable)]
        pub toolbar_model: RefCell<Option<ToolbarModel>>,
        #[template_child]
        pub toolbar_bindings: TemplateChild<glib::BindingGroup>,
        #[template_child]
        pub bookmarks: TemplateChild<BookmarksPage>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub search_toggle_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub status_stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DinghyClient {
        const NAME: &'static str = "DinghyClient";
        type Type = super::DinghyClient;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            glib::BindingGroup::ensure_type();
            BookmarksPage::ensure_type();
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for DinghyClient {
        fn constructed(&self) {
            self.parent_constructed();

            self.toolbar_bindings
                .bind("query", &*self.search_entry, "text")
                .bidirectional()
                .build();
            self.toolbar_bindings
                .bind("searchbar-open", &*self.search_toggle_button, "active")
                .bidirectional()
                .build();
            self.toolbar_bindings
                .bind("searchbar-open", &*self.search_bar, "search-mode-enabled")
                .bidirectional()
                .build();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for DinghyClient {}
    impl BoxImpl for DinghyClient {}
}

glib::wrapper! {
    pub struct DinghyClient(ObjectSubclass<imp::DinghyClient>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl DinghyClient {
    #[template_callback(function)]
    fn get_status_page(available: bool) -> &'static str {
        match available {
            true => "content",
            false => "server-unavailable",
        }
    }

    #[template_callback]
    fn on_retry_clicked(&self) {
        self.imp().status_stack.set_visible_child_name("content");
        self.refresh();
    }

    pub fn bookmarks(&self) -> BookmarksPage {
        self.imp().bookmarks.get()
    }

    pub fn refresh(&self) {
        let _ = self.activate_action("bookmarks.refresh", None);
    }
}
