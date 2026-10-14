use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use linkding::Bookmark;

fn fetch_icon(url: &str) -> Result<Vec<u8>, ureq::Error> {
    static AGENT: std::sync::OnceLock<ureq::Agent> = std::sync::OnceLock::new();
    let agent = AGENT.get_or_init(|| {
        ureq::Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_secs(10)))
            .build()
            .into()
    });
    agent
        .get(url)
        .call()?
        .body_mut()
        .with_config()
        .limit(256 * 256)
        .read_to_vec()
}

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

    impl BookmarkRow {
        pub fn load_icon(&self, url: String) {
            let weak_row = self.downgrade();
            glib::spawn_future_local(async move {
                let Ok(Ok(bytes)) = gio::spawn_blocking(move || fetch_icon(&url)).await else {
                    return;
                };
                let Some(row) = weak_row.upgrade() else {
                    return;
                };
                let Ok(texture) = gtk::gdk::Texture::from_bytes(&glib::Bytes::from_owned(bytes)) else {
                    return;
                };
                row.icon_image.set_paintable(Some(&texture));
                row.icon_image.remove_css_class("dim-label");
            });
        }
    }
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
        if let Some(url) = data
            .favicon_url
            .as_ref()
            .filter(|url| !url.trim().is_empty())
        {
            row.imp().load_icon(url.clone());
        }
        row
    }
}

