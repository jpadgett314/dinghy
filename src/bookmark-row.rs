use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

pub struct BookmarkData {
    pub name: String,
    pub description: String,
    pub icon_path: Option<String>,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "bookmark-row.ui")]
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
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow;
}

impl BookmarkRow {
    pub fn new(data: &BookmarkData) -> Self {
        let row: Self = glib::Object::new();
        row.set_name(&data.name);
        row.set_description(&data.description);

        if let Some(ref path) = data.icon_path {
            let imp = row.imp();
            // TODO: use `path`
            imp.icon_image.set_from_icon_name(Some("text-x-generic-symbolic"));
        }

        row
    }
}
