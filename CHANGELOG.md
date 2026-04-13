# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)  


## [Unreleased]
### Added
- **Git**: Added `.gitignore` file ([4175f53](https://github.com/1estart/esplora-cli/commit/4175f5355423afd6f579e3e02c227c38d3818d3c))

### Changed
- **MSRV**: Updated Minimum Supported Rust Version to `1.74.1` ([3623be6](https://github.com/1estart/esplora-cli/commit/3623be6c402af2f87624941425c6af00a6de9cf9))

- **Dependencies**: Updated multiple core dependencies ([b79b4fa](https://github.com/1estart/esplora-cli/commit/b79b4fa59db1537118f0257bc52a40a9bdf7485c)) via PR [#3](https://github.com/1estart/esplora-cli/pull/3):
  - `hex-conservative`: `0.2.0` → `1.0.0` (API changes applied) ([0488771](https://github.com/1estart/esplora-cli/commit/04887711555ec6a9d717b60779c62975bc990e70))
  - `esplora-client`: `0.9.0` → `0.12.0` (also deprecated methods updated) ([43b970b](https://github.com/1estart/esplora-cli/commit/43b970b29d4fbdd96fd7f9594158aae8cd4e175e))
  - `bitcoin_hashes`: `0.14.0` → `0.20.0` ([f7d288f](https://github.com/1estart/esplora-cli/commit/f7d288f231ffe14c4579997e1e811b7e548066aa))

---

## [0.1.1] - 2024-06-23

### Added
- **CLI**: Initial release with blocking CLI implementation for [rust-esplora-client](https://github.com/bitcoindevkit/rust-esplora-client) ([9e23249](https://github.com/1estart/esplora-cli/commit/9e232491f3ec7e4c79d275fa89f90810bcfbc751))
- **Dependencies**: Added `Cargo.lock` for reproducible builds ([a829079](https://github.com/1estart/esplora-cli/commit/a8290799077437f8c4504b9e3d727ba5a9d4fd6f))
- **Versions**: Bumped cargo package version ([0f1dcca](https://github.com/1estart/esplora-cli/commit/0f1dcca523659356167586781f7c8ebf03d7b506))
