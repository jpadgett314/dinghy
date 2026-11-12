# Release Procedure

1. Add release entry in AppStream metainfo at `data/*.metainfo.xml.in`
2. Update version in Cargo.toml
3. Update version in meson.build
4. Create version tag in git: `v.X.Y.Z` (optionally also `-alpha` or `-beta`; will be marked as pre-release). 
5. Manually dispatch "Create Release Draft" github action. 
6. Review the draft release and publish
7. Update Flathub manifest with the new release URL and SHA256. Currently, a copy exists in this repository.
8. 🥳

# Local Release Testing

1. With native build tools installed, run `just dist`. 
2. Extract the archive and run Meson from that extracted source directory:

```sh
export CARGO_NET_OFFLINE=true
meson setup _build --prefix="$HOME/.local" -Dprofile=default
meson compile -C _build
```

Alternatively, point the release Flatpak manifest at the local archive and build with `flatpak-builder --sandbox`. 

