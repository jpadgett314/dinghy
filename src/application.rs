/* application.rs
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

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib};

use crate::api::Connection;
use crate::config::VERSION;
use crate::views::DinghyWindow;
use crate::views::PreferencesDialog;

mod imp {
    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::DinghyApplication)]
    pub struct DinghyApplication {
        #[property(get)]
        pub connection: Connection,
        #[property(get)]
        pub settings: gio::Settings,
    }

    impl Default for DinghyApplication {
        fn default() -> Self {
            let settings = gio::Settings::new("io.github.jpadgett314.Dinghy");
            let connection = Connection::from(&settings);

            Self { connection, settings }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DinghyApplication {
        const NAME: &'static str = "DinghyApplication";
        type Type = super::DinghyApplication;
        type ParentType = adw::Application;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DinghyApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
            obj.set_accels_for_action("client.refresh", &["<control>r"]);
        }
    }

    impl ApplicationImpl for DinghyApplication {
        fn startup(&self) {
            self.parent_startup();
            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/io/github/jpadgett314/Dinghy/style.css");
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("GTK display should be available"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        fn activate(&self) {
            let application = self.obj();
            let window = application.active_window().unwrap_or_else(|| {
                let window = DinghyWindow::new(&*application, &self.settings, &self.connection);
                window.upcast()
            });

            window.present();
        }
    }

    impl GtkApplicationImpl for DinghyApplication {}
    impl AdwApplicationImpl for DinghyApplication {}
}

glib::wrapper! {
    pub struct DinghyApplication(ObjectSubclass<imp::DinghyApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DinghyApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/io/github/jpadgett314/Dinghy")
            .build()
    }

    fn dinghy_window(&self) -> DinghyWindow {
        self.active_window()
            .and_downcast::<DinghyWindow>()
            .expect("Active window should be a DinghyWindow")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| app.show_preferences())
            .build();
        let onboard_complete_action = gio::ActionEntry::builder("onboard-complete")
            .activate(move |app: &Self, _, _| app.show_client())
            .build();
        let onboard_action = gio::ActionEntry::builder("onboard")
            .activate(move |app: &Self, _, _| app.show_welcome())
            .build();
        self.add_action_entries([quit_action, about_action, preferences_action, onboard_complete_action, onboard_action]);
    }

    fn show_about(&self) {
        let window = self.dinghy_window();

        let about = adw::AboutDialog::builder()
            .application_name("Dinghy")
            .application_icon("io.github.jpadgett314.Dinghy")
            .developer_name("Jacob Padgett")
            .version(VERSION)
            .developers(vec!["Jacob Padgett"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2026 Jacob Padgett")
            .build();

        about.present(Some(&window));
    }

    fn show_preferences(&self) {
        let window = self.dinghy_window();

        let dialog = PreferencesDialog::new();

        dialog.bind_settings(&self.imp().settings);
        dialog.present(Some(&window));
    }

    pub fn show_client(&self) {
        let window = self.dinghy_window();

        window.show_client();
    }

    pub fn show_welcome(&self) {
        let window = self.dinghy_window();

        window.show_welcome();
    }
}
