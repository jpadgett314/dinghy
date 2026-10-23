use std::cell::RefCell;

use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{gio, glib, SearchBar, SearchEntry, ToggleButton};

use crate::bookmark_storage::BookmarkStorage;
use crate::models::connection::{Connection, ConnectionStatus};
use crate::models::BookmarkPagination;
use crate::views::BookmarkFeed;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::DinghyClient)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/client/client.ui")]
    pub struct DinghyClient {
        #[property(get, set)]
        pub settings: RefCell<Option<gio::Settings>>,
        #[property(get, set)]
        pub connection: RefCell<Option<Connection>>,
        #[property(get, nullable)]
        pub bookmark_storage: RefCell<Option<BookmarkStorage>>,
        #[property(get)]
        pub bookmark_pagination: RefCell<Option<BookmarkPagination>>,
        #[template_child]
        pub bookmark_feed_widget: TemplateChild<BookmarkFeed>,
        #[template_child]
        pub search_bar: TemplateChild<SearchBar>,
        #[template_child]
        pub search_entry: TemplateChild<SearchEntry>,
        #[template_child]
        pub search_toggle_button: TemplateChild<ToggleButton>,
        #[template_child]
        pub window_title_widget: TemplateChild<adw::WindowTitle>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DinghyClient {
        const NAME: &'static str = "DinghyClient";
        type Type = super::DinghyClient;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            BookmarkFeed::ensure_type();
            klass.bind_template();
            klass.install_action("client.refresh-bookmarks", None, |client, _, _| {
                client.refresh_bookmarks();
            });
            klass.install_action("client.toggle-search", None, |client, _, _| {
                let imp = client.imp();
                let active = !imp.search_toggle_button.is_active();
                imp.search_toggle_button.set_active(active);
                imp.search_bar.set_search_mode(active);
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for DinghyClient {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.connect_settings_notify(|client| client.setup_data());
            obj.connect_connection_notify(|client| client.setup_data());
            self.search_entry.connect_search_changed(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.refresh_bookmarks();
                    obj.refresh_title();
                }
            ));
            obj.setup_data();
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

impl DinghyClient {
    fn setup_data(&self) {
        if self.bookmark_pagination().is_some() {
            return;
        }
        let (Some(connection), Some(settings)) = (self.connection(), self.settings()) else {
            return;
        };

        let storage = BookmarkStorage::new(connection.api());
        let pagination = BookmarkPagination::new(storage.clone(), settings);
        self.imp().bookmark_storage.replace(Some(storage));
        self.imp().bookmark_pagination.replace(Some(pagination.clone()));
        self.imp().bookmark_feed_widget.set_model(pagination);
        self.notify("bookmark-storage");
        self.notify("bookmark-pagination");
    }

    fn refresh_bookmarks(&self) {
        if let Some(pagination) = self.bookmark_pagination() {
            pagination.fetch_query(self.imp().search_entry.text().to_string());
        }
    }

    fn refresh_title(&self) {
        let (Some(connection), Some(settings)) = (self.connection(), self.settings()) else {
            return;
        };
        let title: String = settings.get("api-url");
        self.imp().window_title_widget.set_title(&title);
        let subtitle = match connection.status() {
            ConnectionStatus::Disconnected => gettext("disconnected"),
            ConnectionStatus::Connected => match connection.stats() {
                None => gettext("disconnected"),
                Some(data) => match data.bookmark_count {
                    0 => gettext("no bookmarks"),
                    1 => gettext("1 bookmark"),
                    n => gettext("{} bookmarks").replace("{}", &n.to_string()),
                },
            },
        };
        self.imp().window_title_widget.set_subtitle(&subtitle);
    }
}
