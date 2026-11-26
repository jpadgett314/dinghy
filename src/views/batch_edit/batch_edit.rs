use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::models::{BatchUpdate, TagOperation};
use crate::widgets::TagEntryRow;
use crate::util::NaturalWidthShim;

type ApplyCallback = Box<dyn FnOnce(BatchUpdate)>;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/batch_edit/batch_edit.ui")]
    pub struct BatchEditDialog {
        pub apply: RefCell<Option<ApplyCallback>>,
        #[template_child]
        pub replace_names: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub name_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub replace_descriptions: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub description_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub tag_operation: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub tags_row: TemplateChild<TagEntryRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BatchEditDialog {
        const NAME: &'static str = "BatchEditDialog";
        type Type = super::BatchEditDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            NaturalWidthShim::ensure_type();
            TagEntryRow::ensure_type();
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BatchEditDialog {
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for BatchEditDialog {}
    impl AdwDialogImpl for BatchEditDialog {}
}

glib::wrapper! {
    pub struct BatchEditDialog(ObjectSubclass<imp::BatchEditDialog>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl BatchEditDialog {
    pub fn new(apply: impl FnOnce(BatchUpdate) + 'static) -> Self {
        let dialog: Self = glib::Object::new();
        dialog.imp().apply.replace(Some(Box::new(apply)));
        dialog
    }

    #[template_callback]
    fn on_cancel_clicked(&self) {
        self.close();
    }

    #[template_callback]
    fn on_apply_clicked(&self) {
        let imp = self.imp();
        let apply = imp.apply.take();
        let Some(apply) = apply else { return };
        imp.tags_row.commit_pending();

        let changes = BatchUpdate {
            title: imp
                .replace_names
                .is_active()
                .then(|| imp.name_row.text().to_string()),
            description: imp
                .replace_descriptions
                .is_active()
                .then(|| imp.description_row.text().to_string()),
            tag_operation: match imp.tag_operation.selected() {
                1 => TagOperation::Remove,
                2 => TagOperation::Replace,
                _ => TagOperation::Add,
            },
            tags: imp
                .tags_row
                .tag_names()
                .iter()
                .map(ToString::to_string)
                .collect(),
        };

        self.close();
        apply(changes);
    }
}
