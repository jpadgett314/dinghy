use std::sync::Mutex;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, PositionType};

use crate::models::BookmarkPagination;
use crate::views::BookmarkCard;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/bookmark_feed/bookmark_feed.ui")]
    pub struct BookmarkFeed {
        pub model: Mutex<BookmarkPagination>,
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
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookmarkFeed {
        fn constructed(&self) {
            self.parent_constructed();

            let model = BookmarkPagination::default();

            self.setup_factory();
            self.setup_infinite_scroll();
            self.setup_click_activation();
            self.selection.set_model(Some(&model));
            self.set_model(model);
        }
    }

    impl BookmarkFeed {
        fn setup_factory(&self) {
            self.factory.connect_setup(|_, item| {
                let item = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem");

                item.set_selectable(false);
                item.set_activatable(true);
            });

            self.factory.connect_bind(|_, item| {
                let item = item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem");

                let card = item
                    .item()
                    .and_downcast::<BookmarkCard>()
                    .expect("model item should be a BookmarkCard");

                item.set_child(Some(&card));
            });

            self.factory.connect_unbind(|_, item| {
                item.downcast_ref::<gtk::ListItem>()
                    .expect("factory item should be a ListItem")
                    .set_child(None::<&gtk::Widget>);
            });
        }

        pub fn setup_click_activation(&self) {
            let weak_list = self.obj().downgrade();

            self.list.connect_activate(move |_, position| {
                let Some(list) = weak_list.upgrade() else {
                    return
                };

                let content = list
                    .imp()
                    .model
                    .lock()
                    .unwrap()
                    .item(position)
                    .and_downcast::<BookmarkCard>();

                if let Some(card) = content {
                    card.open();
                }
            });
        }

        pub fn setup_infinite_scroll(&self) {
            let weak_list = self.obj().downgrade();

            self.scroll_window.connect_edge_reached(move |_, pos| {
                if pos != PositionType::Bottom {
                    return
                }

                let Some(list) = weak_list.upgrade() else {
                    return
                };

                list.imp().model.lock().unwrap().fetch_more();
            });
        }

        pub fn get_model(&self) -> BookmarkPagination {
            self.model.lock().unwrap().clone()
        }

        pub fn set_model(&self, model: BookmarkPagination) {
            let mut model_guard = self.model.lock().unwrap();
            *model_guard = model.clone();
            self.selection.set_model(Some(&model));
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

impl BookmarkFeed {
    pub fn get_model(&self) -> BookmarkPagination {
        self.imp().get_model()
    }

    pub fn set_model(&self, model: BookmarkPagination) {
        self.imp().set_model(model)
    }
}

