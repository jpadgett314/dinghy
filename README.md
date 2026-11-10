<p align="center">
  <img src="data/icons/hicolor/scalable/apps/io.github.jpadgett314.Dinghy.svg" alt="Dinghy app icon" width="128" height="128">
</p>

<h1 align="center">Dinghy</h1>

<p align="center">Bringing Linkding bookmarks to GNOME</p>

<p align="center">
  <img src="data/screenshots/main.png" alt="Dinghy main window">
</p>

Dinghy is a work-in-progress [Linkding](https://github.com/sissbruecker/linkding) client for GNOME, built with Rust, GTK 4, and libadwaita.

You need a running Linkding instance and an API token to connect.

## Installation

Dinghy is not yet available on Flathub. Build from source to try it.

## Building

### GNOME Builder

GNOME Builder is the easiest way to build this project.

1. Download GNOME Builder.
2. In Builder, click "Clone Repository" at the bottom, and enter `https://github.com/jpadgett314/dinghy.git` as the URL.
3. Click the build button at the top once the project is loaded.

### Meson

Requirements:

- Rust 1.92 or newer (including Cargo)
- GTK 4 and its development dependencies for GNOME 50
- libadwaita 1.9 or newer
- Meson 1.0 or newer and Ninja
- gettext

From the project directory:

```sh
meson setup _build --prefix="$HOME/.local"
meson compile -C _build
meson install -C _build
~/.local/bin/dinghy
```

## Acknowledgements

Thank you [Pods](https://github.com/marhkb/pods) for being a great GTK example. Pods also helps me manage my Linkding servers.
