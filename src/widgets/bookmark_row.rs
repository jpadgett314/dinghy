use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use linkding::Bookmark;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "bookmark_row.ui")]
    pub struct BookmarkRow {
        #[template_child]
        pub icon_image: gtk::TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkRow {
        const NAME: &'static str = "BookmarkRow";
        type Type = super::BookmarkRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookmarkRow {}
    impl WidgetImpl for BookmarkRow {}
    impl ListBoxRowImpl for BookmarkRow {}
    impl PreferencesRowImpl for BookmarkRow {}
    impl ActionRowImpl for BookmarkRow {}
}

glib::wrapper! {
    pub struct BookmarkRow(ObjectSubclass<imp::BookmarkRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl BookmarkRow {
    pub fn new(data: &Bookmark) -> Self {
        let row: Self = glib::Object::new();
        let title = data.title.trim();
        row.set_title(if title.is_empty() { &data.url } else { title });
        row.set_subtitle(&data.description.trim());
        row.set_tooltip_text(Some(&data.url));
        row
    }
}
