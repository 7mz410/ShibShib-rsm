# ShibShib rsm changes to VectorCraft

ShibShib rsm is a fork of [VectorCraft](https://github.com/storytold/vectorcraft) by the ArtCraft
team. As the Apache License 2.0 asks, this file lists what the fork changes. Upstream code keeps
its copyright and licence (MIT OR Apache-2.0); see `NOTICE`.

## Branding

- App name in the interface, window title, macOS menus and About box: "ShibShib rsm".
- App icon: the ShibShib rsm mark (`docs/shibshib/`), regenerated with `packaging/icons.sh`.
- macOS bundle name and identifier (`com.shibshib.rsm`).
- Help menu links point to the ShibShib rsm repository.
- Upstream's ArtCraft brand files (`docs/brand/`) are removed.
- The upstream community Discord button (title bar, start screen) and the ArtCraft website link are
  removed; the start screen links to this repository.

The rename is applied by `shibshib/rebrand.py` and touches user-visible text only. Crate names, the
`.vectorcraft` file format and other internals keep their upstream names so upstream updates merge
cleanly.

## Updating from upstream

```sh
git fetch upstream
git merge upstream/main          # resolve conflicts in branded strings in favour of upstream
python3 shibshib/rebrand.py      # re-apply the branding
```

## Arabic interface

- Arabic (`ar`) added to the interface languages, with a catalog in `crates/ui-egui/src/i18n/ar.tsv`
  (being completed) and Arabic plural rules.
- IBM Plex Sans Arabic (OFL) added to the UI fonts so Arabic draws without system fonts, including
  in the web build.
- `crates/ui-egui/src/i18n/bidi.rs` stores right-to-left translations in visual order, because egui
  shapes words but lays them out left to right. Sizes such as 1920×1080 stay left to right, and
  the leading and trailing spaces of joined fragments stay in place.
- The Help/Community "Website" link reads "ShibShib Website" and opens https://shibshib.art; the
  GitHub link still opens this repository (`rebrand.py`, `SITE`).
