# Changelog

## [0.3.1](https://github.com/vipsa-ab/backend/compare/v0.3.0...v0.3.1) (2026-05-10)


### Bug Fixes

* update Dockerfile for dynamic target detection and static binary copying ([7169b70](https://github.com/vipsa-ab/backend/commit/7169b70760ea48e8827635ab3c5684ed54c4ef2a))
* update Dockerfile for dynamic target detection and static binary… ([e6c6918](https://github.com/vipsa-ab/backend/commit/e6c6918f0e97dd6aff8aa1d23bdf3fe82dae52e3))

## [0.3.0](https://github.com/vipsa-ab/backend/compare/v0.2.1...v0.3.0) (2026-05-09)


### Features

* add configurable CORS support ([1e85218](https://github.com/vipsa-ab/backend/commit/1e852181d4503d7e18cead0a8b55271d58850df1))
* add configurable CORS support ([12cfce5](https://github.com/vipsa-ab/backend/commit/12cfce533d9d16a1eaf36b5f94df945a47c646f3))


### Bug Fixes

* use release-type rust for proper Cargo.toml versioning ([e5142aa](https://github.com/vipsa-ab/backend/commit/e5142aa280eb64d7d8b4a05a4d5f902b77449210))

## [0.2.1](https://github.com/vipsa-ab/backend/compare/v0.2.0...v0.2.1) (2026-05-09)


### Bug Fixes

* use std::env::var instead of config crate ([0ea745d](https://github.com/vipsa-ab/backend/commit/0ea745d88cd06d5b06f17c9a3f7d762962343116))
* use std::env::var instead of config crate ([9a3bfde](https://github.com/vipsa-ab/backend/commit/9a3bfde40397b4cece61b90de2e25fa2e5538396)), closes [#4](https://github.com/vipsa-ab/backend/issues/4)

## [0.2.0](https://github.com/vipsa-ab/backend/compare/v0.1.0...v0.2.0) (2026-05-09)


### Features

* add dotenvy support for .env configuration ([c3d39b2](https://github.com/vipsa-ab/backend/commit/c3d39b26e76385ee92e5bd96e90192edcc8dbc4c)), closes [#4](https://github.com/vipsa-ab/backend/issues/4)

## [0.1.0](https://github.com/vipsa-ab/backend/compare/v0.0.0...v0.1.0) (2026-04-27)


### Features

* **docker:** add scratch runtime for minimal image size ([adfe6ef](https://github.com/vipsa-ab/backend/commit/adfe6ef449f8708776f11abcde40e6e4c8e4957a)), closes [#1](https://github.com/vipsa-ab/backend/issues/1)
* **docker:** scratch runtime para imagen minimal (~24MB) ([21e0b73](https://github.com/vipsa-ab/backend/commit/21e0b73af2ecbf3da8209e21ebf92dff0c19a4b9))

## [0.1.1](https://github.com/vipsa-ab/backend/compare/v0.1.0...v0.1.1) (2026-04-27)


### Bug Fixes

* **ci:** remove working-directory defaults, run from repo root ([66e9e34](https://github.com/vipsa-ab/backend/commit/66e9e34fb7abe521db61be888aadca025f8b4a94))
* **ci:** use defaults.run.working-directory instead of step-level ([804b516](https://github.com/vipsa-ab/backend/commit/804b5161d736743433c592989888464d2aa52397))
* **ci:** use hashFiles() instead of hashOf() ([4a78871](https://github.com/vipsa-ab/backend/commit/4a788715340ba7246ec139ae0b513c23363da048))
* **clippy:** add #[allow(dead_code)] to impl blocks and methods ([192fde0](https://github.com/vipsa-ab/backend/commit/192fde0e889d37904dce4308f79596070989343d))
* **clippy:** allow dead code for unused structs and fns ([2fdfdf0](https://github.com/vipsa-ab/backend/commit/2fdfdf008a9fcc951ff82bd9f1436a4f4578535c))
* **clippy:** prefix unused validation msg with underscore ([c9f426b](https://github.com/vipsa-ab/backend/commit/c9f426b071eb2327a7c0cc55483466724935dd5d))

## 0.1.0 (2026-04-27)


### Features

* add GitHub Actions for CI and release-please ([2613df0](https://github.com/vipsa-ab/backend/commit/2613df0a149daafd85f489c4e8bbd07e71f33d24))
* initial project setup with hexagonal architecture ([ce33b62](https://github.com/vipsa-ab/backend/commit/ce33b6260bfacfd6808629456015408e6a2ec425))
