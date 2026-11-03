use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::widgets::TagChip;

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::TagEntryRow)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/widgets/tag_entry_row/tag_entry_row.ui")]
    pub struct TagEntryRow {
        #[property(get, set = Self::set_tag_names)]
        pub tag_names: RefCell<glib::StrV>,
        #[template_child]
        pub entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub tags_box: TemplateChild<adw::WrapBox>,
    }

    impl TagEntryRow {
        fn set_tag_names(&self, names: glib::StrV) {
            self.tags_box.remove_all();
            for name in &names {
                self.tags_box.append(&TagChip::new(name));
            }
            self.tags_box.set_visible(!names.is_empty());
            self.tag_names.replace(names);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TagEntryRow {
        const NAME: &'static str = "TagEntryRow";
        type Type = super::TagEntryRow;
        type ParentType = adw::PreferencesRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
            klass.install_action(
                "tag-entry.remove",
                Some(glib::VariantTy::STRING),
                |row, _, parameter| {
                    let name = parameter.unwrap().get::<String>().unwrap();
                    let names: Vec<String> = row
                        .tag_names()
                        .iter()
                        .filter(|tag| tag.as_str() != name)
                        .map(|tag| tag.to_string())
                        .collect();
                    row.set_tag_names(glib::StrV::from(names));
                },
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for TagEntryRow {
        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WidgetImpl for TagEntryRow {}
    impl ListBoxRowImpl for TagEntryRow {}
    impl PreferencesRowImpl for TagEntryRow {}
}

glib::wrapper! {
    pub struct TagEntryRow(ObjectSubclass<imp::TagEntryRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

#[gtk::template_callbacks]
impl TagEntryRow {
    #[template_callback]
    pub fn commit_pending(&self) {
        let text = self.imp().entry.text();
        let name = text.trim();
        if name.is_empty() {
            return;
        }
        let mut names: Vec<String> = self.tag_names().iter().map(|tag| tag.to_string()).collect();
        if !names.iter().any(|tag| tag == name) {
            names.push(name.to_owned());
            self.set_tag_names(glib::StrV::from(names));
        }
        self.imp().entry.set_text("");
    }

    #[template_callback(function)]
    fn has_text(text: &str) -> bool {
        !text.trim().is_empty()
    }
}
