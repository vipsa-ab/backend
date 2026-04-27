# Changelog

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
