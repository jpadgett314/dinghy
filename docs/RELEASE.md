# Release Procedure

## 1. Bump Version

Update version in `Cargo.toml` and `meson.build` (identical version strings). 

Format: `X.Y.Z[-modifier.#]`

Ensure the change is reflected in `Cargo.lock` by running, for example, `cargo check`. 

## 2. Record release in AppStream metainfo

Manually add a new `<release>` object in `data/io.github.jpadgett314.Dinghy.metainfo.xml.in`. 

Format: `X.Y.Z[~modifier.#]`

> [!NOTE]
> Prerelease version modifiers must use the tilde (~) to ensure that stable versions supercede them.

Entries with with prerelease modifiers (`-alpha`, `-beta`) should be collapsed into the next stable release entry.

## 3. Tag in Git

Commit changes, then tag the latest commit with the version in git, e.g. `git tag 'v1.2.3'`. 

Format: `X.Y.Z[-modifier.#]`

## 4. Publish to Github

In GitHub, run the "Create Release Draft" action against the latest tag.

Find the release draft. Edit it, then publish. 

## 5. Publish to Flathub

⚠️ WORK IN PROGRESS ⚠️

## 6. Upload Bundle

Locate the vendored archive attached to the GitHub release.

Record the vendored archive URL and SHA256 in the release manifest `io.github.jpadgett314.Dinghy.json`.

Generate the .flatpak bundle: 

```
just --set profile release bundle
```

Finally, attach the bundle to the latest GitHub release.

## 7. 🥳

# Local Release Testing 

Generate the vendored archive locally:

```
just --set profile release dist
```

Manually point the release manifest at the local archive and build:

```
just --set profile release bundle
```

Alternatively, just change the build configuration in the development manifest to `default` temporarily.

