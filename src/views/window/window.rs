/* window.rs
 *
 * Copyright 2026 Jacob Padgett
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::api::Connection;
use crate::views::{DinghyClient, DinghyWelcome};

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::DinghyWindow)]
    #[template(resource = "/io/github/jpadgett314/Dinghy/views/window/window.ui")]
    pub struct DinghyWindow {
        #[property(get, set, construct)]
        pub settings: RefCell<Option<gio::Settings>>,
        #[property(get, set, construct)]
        pub connection: RefCell<Option<Connection>>,
        #[template_child]
        pub client: TemplateChild<DinghyClient>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DinghyWindow {
        const NAME: &'static str = "DinghyWindow";
        type Type = super::DinghyWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            DinghyClient::ensure_type();
            DinghyWelcome::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for DinghyWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            if obj.settings().is_some_and(|p| p.get("onboarded")) {
                obj.show_client();
            } else {
                obj.show_welcome();
            }
        }
    }

    impl WidgetImpl for DinghyWindow {}
    impl WindowImpl for DinghyWindow {}
    impl ApplicationWindowImpl for DinghyWindow {}
    impl AdwApplicationWindowImpl for DinghyWindow {}
}

glib::wrapper! {
    pub struct DinghyWindow(ObjectSubclass<imp::DinghyWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl DinghyWindow {
    fn setup_gactions(&self) {
        let toast_action = gio::ActionEntry::builder("toast")
            .parameter_type(Some(glib::VariantTy::STRING))
            .activate(move |win: &Self, _, param| win.on_toast_action(param))
            .build();

        self.add_action_entries([toast_action]);

        self.insert_action_group("client", Some(&self.imp().client.actions()));
        self.insert_action_group("bookmarks", Some(&self.imp().client.bookmarks().actions()));
    }

    fn on_toast_action(&self, param: Option<&glib::Variant>) {
        let title = param
            .unwrap()
            .get::<String>()
            .unwrap();

        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new(&title));
    }

    pub fn new<P: IsA<gtk::Application>>(
        application: &P,
        settings: &gio::Settings,
        connection: &Connection,
    ) -> Self {
        glib::Object::builder()
            .property("application", application)
            .property("settings", settings)
            .property("connection", connection)
            .build()
    }

    pub fn show_client(&self) {
        self.imp()
            .stack
            .set_visible_child_name("client");

        self.imp()
            .client
            .refresh();
    }

    pub fn show_welcome(&self) {
        self.imp()
            .stack
            .set_visible_child_name("welcome");
    }
}
