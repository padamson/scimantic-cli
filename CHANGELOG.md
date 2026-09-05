# Changelog

All notable changes to this project are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- `dependabot-auto-merge.yml`: enables squash auto-merge on every Dependabot PR; the required status checks are the whole gate, since merges made with `GITHUB_TOKEN` trigger no push run on `main`
- Dependabot `cooldown` (7/14/7/3 days for cargo, 7 for actions) so bumps arrive after the cargo-vet import sets have audited them
- Dependabot groups for majors and for actions, so the weekly run opens at most three PRs and the auto-merge cascade cannot start

## [0.1.0] - 2026-04-19

### Added
- Initial release of the `scimantic` CLI.
- `scimantic --version` / `-V` prints `scimantic 0.1.0`.
- `scimantic --help` / `-h` prints brief usage.

This release is deliberately minimal: it reserves the crate name on crates.io,
validates the end-to-end release pipeline, and gives early adopters a binary
they can `cargo install` ahead of feature work landing in later releases.

[Unreleased]: https://github.com/padamson/scimantic-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/padamson/scimantic-cli/releases/tag/v0.1.0
