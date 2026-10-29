use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::{gettext, ngettext};
use gtk::{gio, glib, SearchBar, SearchEntry, ToggleButton};

use crate::api::BookmarkStorage;
use crate::models::Connection;
use crate::models::BookmarkFeedModel;
use crate::views::{CreateBookmarkView, BookmarkFeed};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::DinghyClient)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/client/client.ui")]
    pub struct DinghyClient {
        #[property(get, nullable)]
        pub actions: RefCell<Option<gio::SimpleActionGroup>>,
        #[property(get, set, nullable)]
        pub connection: RefCell<Option<Connection>>,
        #[property(get, set, nullable)]
        pub settings: RefCell<Option<gio::Settings>>,
        #[property(get, nullable)]
        pub bookmark_storage: RefCell<Option<BookmarkStorage>>,
        #[property(get, nullable)]
        pub bookmark_feed_model: RefCell<Option<BookmarkFeedModel>>,
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
            self.obj().setup_gactions();
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
    fn setup_gactions(&self) {
        type ActionBuilder = gio::ActionEntryBuilder::<gio::SimpleActionGroup>;

        let actions = &self.imp().actions;
        let client1 = self.clone();
        let client2 = self.clone();
        let entries = [
            ActionBuilder::new("refresh-bookmarks")
                .activate(move |_, _, _| client1.refresh())
                .build(),
            ActionBuilder::new("toggle-search")
                .activate(move |_, _, _| client2.on_toggle_search())
                .build(),
        ];

        actions.replace(Some(gio::SimpleActionGroup::new()));

        actions.borrow().as_ref().unwrap().add_action_entries(entries);
    }

    fn on_toggle_search(&self) {
        let imp = self.imp();
        let active = !imp.search_toggle_button.is_active();
        imp.search_toggle_button.set_active(active);
        imp.search_bar.set_search_mode(active);
    }

    #[template_callback]
    fn on_create_bookmark_clicked(&self) {
        if let Some(storage) = self.bookmark_storage() {
            CreateBookmarkView::new(&storage).present(Some(self));
        }
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
        let pagination = BookmarkFeedModel::new(storage.clone(), settings);
        self.imp().bookmark_storage.replace(Some(storage));
        self.imp().bookmark_feed_model.replace(Some(pagination.clone()));
        self.imp().bookmark_feed_widget.set_model(pagination);

        self.refresh();
    }

    #[template_callback]
    fn on_search_changed(&self) {
        self.refresh();
    }

    fn refresh_bookmarks(&self) {
        if let Some(pagination) = self.bookmark_feed_model() {
            pagination.fetch_query(self.imp().search_entry.text().to_string());
        }
    }

    fn refresh_title(&self) {
        let (Some(settings), Some(connection)) = (self.settings(), self.connection()) else {
            return;
        };

        let title: String = settings.get("api-url");
        let title_widget = self.imp().window_title_widget.get();

        glib::spawn_future_local(async move {
            let reply =
                gio::spawn_blocking(move || connection.stats())
                    .await;

            let count = match reply {
                Err(_) => gettext("Not available"),
                Ok(None) => gettext("Not available"),
                Ok(Some(data)) => match data.bookmark_count {
                    0 => gettext("No bookmarks"),
                    n => ngettext("{} bookmark", "{} bookmarks", n as u32).replace("{}", &n.to_string()),
                }
            };

            title_widget.set_title(&title);
            title_widget.set_subtitle(&count);
        });
    }

    pub fn refresh(&self) {
        self.refresh_bookmarks();
        self.refresh_title();
    }
}
