# Changelog

## [1.1](https://github.com/Coddeus/xinhua-zhaodao/releases/tag/v1.1) - 2023-04-23
### Added
- GUI with Tauri and Trunk.
- Basic app CSS style.
- Search for derived words 词(ci).
- Search results automatic update when input changes.
- Error handling for buggy search cases.
- Chinese as the default language of this app.
- App "Zhong" icon.
- `.vscode` dev setup
### Changed
- Repo structure following Tauri arrival.
- Version tags from "X.Y" to "vX.Y".
- Development branch from "main" to "dev", "main" is now the release branch.
### Removed
- Console input-output.
### Fixed
- Release binaries publishing with Github Actions; now based on tauri-action.

## [1.0](https://github.com/Coddeus/xinhua-zhaodao/releases/tag/v1.0) - 2023-04-16
### Added
- `data/hanyu.db`, Chinese language database.
- Connection to this DB from `src/main.rs` with sqlite.
- User terminal app to look up derived characters 字(zi).
- Github Actions on `push` and `release` (the latter doesn't work).
- All repo information in `README.md`.
- This `CHANGELOG.md`.