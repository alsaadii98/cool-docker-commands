# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-08-17

### Fixed

- `--demo` panicked with `SocketNotFoundError` on machines with no Docker
  socket, which was the one case it exists to serve. It now builds an offline
  client instead of probing the filesystem.
- `dok top --demo` printed a daemon error per container; it now has fixtures
  like every other demo command.

### Changed

- Homebrew installs a prebuilt binary instead of compiling from source, so it
  no longer pulls rust, llvm, z3 and python. `brew install --HEAD` still builds
  from source.

## [0.1.0] - 2026-08-17

First public release.

### Added

- `ps` (`ls`) — containers grouped by compose project, with short ids, state
  dots, health marks, `:8080→80` port mapping, compacted status and ages.
- `images` (`img`) — size and age gradients, dangling images marked
  reclaimable, per-image container counts.
- `df` (`du`) — disk usage per category with used/reclaimable bars, and
  `-v` for the biggest images, containers, volumes and build-cache entries.
- `inspect` — inspect JSON folded into identity/state/config/resources/
  network/mounts/labels, with credential-looking env values masked.
- `logs` — merged multi-container tail, stable per-container colours, stderr
  marked on the separator, level tokens and JSON lines highlighted.
- `top` — processes inside containers nested by parent PID.
- `tree` — compose projects, networks with container IPs, volumes with mount
  points.
- `stats` — live CPU/memory/IO dashboard with a total-CPU sparkline.
- `events` — colour-coded daemon event stream, `exec_*` hidden by default.
- `themes` — 10 built-in themes carrying palette, glyph set and layout, plus
  `--preview` and `--init`.
- Configuration via `~/.config/dok/config.toml`, including custom themes that
  inherit from a built-in.
- `--demo` on every command: renders a built-in example stack with no daemon,
  and is what generates the screenshots in the docs.

[Unreleased]: https://github.com/alsaadii98/cool-docker-commands/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/alsaadii98/cool-docker-commands/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/alsaadii98/cool-docker-commands/releases/tag/v0.1.0
