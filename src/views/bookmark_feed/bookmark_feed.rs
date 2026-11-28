use std::cell::{Cell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::models::{BookmarkCardModel, BookmarkFeedModel};
use crate::views::BookmarkCard;

mod imp {
    use super::*;

    #[derive(Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::BookmarkFeed)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmark_feed/bookmark_feed.ui")]
    pub struct BookmarkFeed {
        #[property(get, set)]
        pub model: RefCell<BookmarkFeedModel>,
        #[property(get)]
        pub selected_count: Cell<u32>,
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListView>,
        #[template_child]
        pub select_focused_shortcut: gtk::TemplateChild<gtk::Shortcut>,
        #[template_child]
        pub selection: gtk::TemplateChild<gtk::MultiSelection>,
        #[template_child]
        pub scroll_window: gtk::TemplateChild<gtk::ScrolledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkFeed {
        const NAME: &'static str = "BookmarkFeed";
        type Type = super::BookmarkFeed;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            BookmarkCard::ensure_type();
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkFeed {
        fn constructed(&self) {
            self.parent_constructed();
            self.select_focused_shortcut.set_action(Some(
                gtk::CallbackAction::new(
                    |widget, _| {
                        widget.downcast_ref::<super::BookmarkFeed>().map_or(
                            glib::Propagation::Proceed,
                            super::BookmarkFeed::on_select,
                        )
                    }
                )
            ));
        }

        fn dispose(&self) {
            self.dispose_template();
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

#[gtk::template_callbacks]
impl BookmarkFeed {
    #[template_callback]
    fn on_edge_reached(&self, edge: gtk::PositionType) {
        if edge == gtk::PositionType::Bottom {
            self.model().fetch_more();
        }
    }

    #[template_callback]
    fn on_selection_changed(&self) {
        let selection = self.imp().selection.selection();
        self.imp().selected_count.replace(selection.size() as u32);
        self.notify_selected_count();
    }

    #[template_callback]
    fn on_activate(&self, position: u32) {
        if let Some(bookmark) = self
            .model()
            .item(position)
            .and_downcast::<BookmarkCardModel>()
        {
            bookmark.open(self.root().and_downcast::<gtk::Window>().as_ref());
        }
    }

    fn on_select(&self) -> glib::Propagation {
        if let Some(card) = self
            .imp()
            .list
            .focus_child()
            .filter(|row| row.is_focus())
            .and_then(|row| row.first_child())
            .and_downcast::<BookmarkCard>()
        {
            card.toggle_selection();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    }
}

impl BookmarkFeed {
    pub fn selected_bookmarks(&self) -> Vec<BookmarkCardModel> {
        let selection = self.imp().selection.clone();

        (0..selection.n_items())
            .filter(|position| selection.is_selected(*position))
            .filter_map(|position| selection.item(position))
            .filter_map(|item| item.downcast::<BookmarkCardModel>().ok())
            .collect()
    }

    pub fn select_all(&self) {
        self.imp().selection.select_all();
    }

    pub fn clear_selection(&self) {
        self.imp().selection.unselect_all();
    }
}
