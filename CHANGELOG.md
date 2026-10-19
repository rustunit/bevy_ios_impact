# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.9.0] - 2026-10-19

### Changed
* upgrade to bevy `0.20`

## [0.8.0] - 2026-08-24

### Changed
* **BREAKING:** `ImpactResource` is a non-send resource now - take it as
  `NonSendMut<ImpactResource>` instead of `ResMut<ImpactResource>`. UIKit's feedback
  generators are main-thread-only, which the old bindings did not model.
* use `objc2-ui-kit`'s generated bindings instead of the companion `ios_impact` crate

### Removed
* the companion `ios_impact` crate

## [0.7.0] - 2026-08-09

### Changes
* support Bevy `0.19`

## [0.6.0] - 2026-01-16

### Changes
* support Bevy `0.18`
 
## [0.5.0] - 2025-10-18

### Changes
* support Bevy `0.17`

## [0.4.0] - 2025-04-26

### Changes
* support Bevy `0.16`

## [0.3.0] - 2024-12-02

### Changes
* support Bevy `0.15`

## [0.2.0] - 2024-07-07

### Changes
* support Bevy `0.14`

## [0.1.3] - 2024-01-15

### Fixes
* `0.1.2` broke build on ios

## [0.1.2] - 2024-01-15

### Fixes
* allow compiling on wasm
