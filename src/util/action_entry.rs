use gtk::glib::prelude::*;
use gtk::{gio, glib};

pub fn action_entry0<O: IsA<glib::Object> + 'static>(
    object: &O,
    name: &str,
    activate: impl Fn(&O) + 'static,
) -> gio::ActionEntry<gio::SimpleActionGroup> {
    let object = object.downgrade();
    gio::ActionEntry::builder(name)
        .activate(move |_: &gio::SimpleActionGroup, _, _| {
            if let Some(object) = object.upgrade() {
                activate(&object);
            }
        })
        .build()
}

pub fn action_entry1<O: IsA<glib::Object> + 'static>(
    object: &O,
    name: &str,
    parameter_type: Option<&glib::VariantTy>,
    activate: impl Fn(&O, Option<&glib::Variant>) + 'static,
) -> gio::ActionEntry<gio::SimpleActionGroup> {
    let object = object.downgrade();
    gio::ActionEntry::builder(name)
        .parameter_type(parameter_type)
        .activate(move |_: &gio::SimpleActionGroup, _, parameter| {
            if let Some(object) = object.upgrade() {
                activate(&object, parameter);
            }
        })
        .build()
}

