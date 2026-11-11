app_id := 'io.github.jpadgett314.Dinghy.Devel'
manifest := app_id + '.json'
branch := 'devel'
bundle_file := 'dinghy-devel.flatpak'

default:
    @just --list

# Install the Flatpak SDK, runtime, and Rust extension from Flathub.
setup:
    flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
    flatpak-builder --user --install-deps-from=flathub --install-deps-only .flatpak-build {{manifest}}

# Build the current checkout and export it to a local Flatpak repository.
build:
    flatpak-builder --user --force-clean --default-branch={{branch}} --repo=.flatpak-repo .flatpak-build {{manifest}}

# Build and install the development app, replacing any existing copy.
install: build
    flatpak install --user --reinstall ./.flatpak-repo {{app_id}}//{{branch}}

# Run the installed development app.
run:
    flatpak run {{app_id}}//{{branch}}

# Build an installable bundle, like the CI artifact.
bundle: build
    flatpak build-bundle --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo .flatpak-repo {{bundle_file}} {{app_id}} {{branch}}

# Install a downloaded or locally built bundle (without rebuilding).
install-bundle path=bundle_file:
    flatpak install --user --reinstall "{{path}}"

# Configure a native build with the regular application ID.
native-setup:
    meson setup _build --prefix="$HOME/.local"

# Compile the configured native build.
native-build:
    meson compile -C _build

# Build and install the native app.
native-install: native-build
    meson install -C _build

# Run the native app installed by native-install.
native-run:
    "$HOME/.local/bin/dinghy"
