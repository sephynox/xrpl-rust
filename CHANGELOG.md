# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [[Incomplete]]

- Models
- Integration Tests
- Performance Benchmarks

## [[Unreleased]]

## [[v0.4.0]]

- add amm support
  - Transaction models
  - Transaction signing
  - Request models
- add sidechain support
  - Transaction models
  - Transaction signing
- improve errorhandling utilizing thiserror
- simplifying feature flags

## [[v0.3.0]]

- Examples
  - Wallet from seed
  - New wallet generation
  - Client requests
- make `new` methods of models public
- add `AsyncWebSocketClient` and `WebSocketClient`
- add `AsyncJsonRpcClient` and `JsonRpcClient`
- update dependencies
- add devcontainer
- add transaction helpers and signing
- add account helpers
- add ledger helpers
- add wallet helpers

---

## [[v0.2.0-beta]]

### Added

- Request models
- Transaction models
- Ledger models
- Utilize `anyhow` and `thiserror` for models
- Utilities regarding `serde` crate
- Utilities regarding `anyhow` crate

### Changed

- Use `serde_with` to reduce repetitive serialization skip attribute tags
- Use `strum_macros::Display` instead of manual `core::fmt::Display`
- Use `strum_macros::Display` for `CryptoAlgorithm` enum
- Separated `Currency` to `Currency` (`IssuedCurrency`, `XRP`) and `Amount` (`IssuedCurrencyAmount`, `XRPAmount`)
- Make `Wallet` fields public
- Updated crates:
  - secp256k1
  - crypto-bigint
  - serde_with
  - criterion

### Fixed

- Broken documentation link
- Flatten hex exceptions missed from previous pass

---

## [v0.1.1] - 2021-10-28

Initial core release.

### Added

- All Core functionality working with unit tests
