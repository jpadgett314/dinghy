use std::cell::RefCell;

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
        #[template_child]
        pub list: gtk::TemplateChild<gtk::ListView>,
        #[template_child]
        pub selection: gtk::TemplateChild<gtk::NoSelection>,
        #[template_child]
        pub factory: gtk::TemplateChild<gtk::SignalListItemFactory>,
        #[template_child]
        pub scroll_window: gtk::TemplateChild<gtk::ScrolledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkFeed {
        const NAME: &'static str = "BookmarkFeed";
        type Type = super::BookmarkFeed;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BookmarkFeed {
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
    fn on_setup(&self, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        item.set_selectable(false);
        item.set_activatable(true);
        item.set_child(Some(&BookmarkCard::default()));
    }

    #[template_callback]
    fn on_bind(&self, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let card = item.child().and_downcast::<BookmarkCard>().unwrap();
        card.set_model(item.item().and_downcast::<BookmarkCardModel>().as_ref());
    }

    #[template_callback]
    fn on_unbind(&self, item: &glib::Object) {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let card = item.child().and_downcast::<BookmarkCard>().unwrap();
        card.set_model(None::<&BookmarkCardModel>);
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
}
