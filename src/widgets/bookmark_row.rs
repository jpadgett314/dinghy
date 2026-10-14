use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use std::cell::OnceCell;

use crate::bookmark_record::BookmarkRecord;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "bookmark_row.ui")]
    pub struct BookmarkRow {
        pub bookmark: OnceCell<BookmarkRecord>,
        #[template_child]
        pub url_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub title_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub description_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub icon_image: gtk::TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BookmarkRow {
        const NAME: &'static str = "BookmarkRow";
        type Type = super::BookmarkRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BookmarkRow {}
    impl WidgetImpl for BookmarkRow {}
    impl BoxImpl for BookmarkRow {}
}

glib::wrapper! {
    pub struct BookmarkRow(ObjectSubclass<imp::BookmarkRow>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BookmarkRow {
    pub fn new(bookmark: &BookmarkRecord) -> Self {
        let row: Self = glib::Object::new();

        let _ = row.imp().bookmark.set(bookmark.clone());
        row.imp().url_label.set_label(&bookmark.url);
        row.imp().title_label.set_label(&bookmark.title);
        row.imp().description_label.set_label(&bookmark.description);

        row.set_tooltip_text(Some(&bookmark.url));

        let url = bookmark
            .favicon_url
            .as_ref()
            .filter(|url| !url.trim().is_empty());

        if let Some(url) = url {
            row.load_icon(url.clone());
        }

        row
    }

    pub fn open(&self) {
        let Some(bookmark) = &self.imp().bookmark.get() else {
            return;
        };
        let parent = self.root().and_downcast::<gtk::Window>();
        let opener = gtk::UriLauncher::new(&bookmark.url);
        opener.launch(
            parent.as_ref(),
            None::<&gio::Cancellable>,
            |result| {
                if let Err(error) = result {
                    eprintln!("Could not open bookmark: {error}");
                }
            }
        );
    }

    pub fn load_icon(&self, url: String) {
        let weak_row = self.downgrade();
        glib::spawn_future_local(async move {
            let Ok(Ok(bytes)) = gio::spawn_blocking(move || fetch_icon(&url)).await else {
                return;
            };
            let Ok(texture) = gtk::gdk::Texture::from_bytes(&glib::Bytes::from_owned(bytes)) else {
                return;
            };
            let Some(row) = weak_row.upgrade() else {
                 return;
            };

            row.imp().icon_image.set_paintable(Some(&texture));
            row.imp().icon_image.remove_css_class("dim-label");
        });
    }
}

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

