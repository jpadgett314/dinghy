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
use linkding::LinkDingClient;

use crate::models::BookmarkModel;
use crate::config::VERSION;
use crate::widgets::PreferencesDialog;
use crate::DinghyWindow;

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct DinghyApplication {
        pub settings: gio::Settings,
        pub bookmark_model: BookmarkModel,
    }

    impl Default for DinghyApplication {
        fn default() -> Self {
            let settings = gio::Settings::new("io.github.jpadgett314.Dinghy");
            let url: String = settings.get("server-url");
            let tok: String = settings.get("api-token");
            let linkding_api = LinkDingClient::new(&url, &tok);
            let bookmark_model = BookmarkModel::new(linkding_api);

            /*
            settings.connect_changed(Some("server-url"), move |settings, key| {
                replace_api()
            });
            settings.connect_changed(Some("api-key"), move |settings, key| {
                replace_api()
            });
            */

            Self { settings, bookmark_model }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DinghyApplication {
        const NAME: &'static str = "DinghyApplication";
        type Type = super::DinghyApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for DinghyApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for DinghyApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = DinghyWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
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
        let refresh_bookmarks_action = gio::ActionEntry::builder("refresh-bookmarks")
            .activate(move |app: &Self, _, _| app.refresh_bookmarks())
            .build();
        self.add_action_entries([quit_action, about_action, preferences_action, refresh_bookmarks_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
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
        let window = self.active_window().unwrap();
        let dialog = PreferencesDialog::new();

        dialog.bind_settings(&self.imp().settings);
        dialog.present(Some(&window));
    }

    fn refresh_bookmarks(&self) {
        let window = self
            .active_window()
            .and_downcast::<crate::window::DinghyWindow>()
            .expect("Active window should be a DinghyWindow");

        let bookmarks = self.imp().bookmark_model.search(String::from(""), 0);

        window.imp().bookmark_list_widget.replace(&bookmarks);
    }
}
