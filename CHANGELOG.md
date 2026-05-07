# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/LDeakin/openzl-sys/compare/v0.2.0+openzl.0.2.0...HEAD)

## [0.2.0+openzl.0.2.0](https://github.com/LDeakin/openzl-sys/releases/tag/v0.2.0+openzl.0.2.0) - 2026-05-08

### Added
- Bindings for OpenZL 0.2.0 additions, including SDDL2/runtime graph support, native LZ and LZ4 graph helpers, automatic chunking constants, dictionary and materialization APIs, header comments, graph-depth/error-context helpers, decompression introspection hooks, and node dictionary/MParam query helpers.

### Changed
- Bump MSRV to 1.77.
- Bump `openzl` to 0.2.0. This pulls in the upstream SDDL2 rewrite, native LZ codec, very-large-input chunking support, expanded codec catalog, tighter `ZL_compressBound()`, and public API cleanup.
- **Breaking**: Generated bindings reflect the OpenZL 0.2.0 public API cleanup. Notable source breaks include `ZL_Data_s` becoming `Stream_s`, `ZL_HAVE_FBCODE` becoming `ZL_IS_FBCODE`, `ZL_E_addFrame_public()` becoming `ZL_E_addFrame()`, `ZL_PipeDstCapacityFn` splitting into compression/decompression variants, descriptor structs gaining dictionary/materializer/MParam fields, `ZL_CompressIntrospectionHooks_s` layout changes, and removal of `ZL_Compressor_cloneNode()`, `ZL_reportError()`, and `ZL_ENABLE_RET_IF_ARG_PRINTING`.

## [0.1.2+openzl.0.1.0] - 2025-10-08

### Changed
 - Use HTTPS for submodule over SSH

### Fixed
 - Fix trusted publishing
 - Compile on windows-msvc with clang-cl
 - Remove unnecessarily packaged files

## [0.1.1+openzl.0.1.0] - 2025-10-07

### Added
 - Add trusted publishing

### Fixed
 - Switch OpenZL to v0.1.0 tag (from `dev`)

## [0.1.0+openzl.0.1.0] - 2025-10-07

### Added
 - Initial release
 - OpenZL: **0.1.0** (2025-10-07)

[0.1.2+openzl.0.1.0]: https://github.com/LDeakin/openzl-sys/releases/tag/v0.1.2+openzl.0.1.0
[0.1.1+openzl.0.1.0]: https://github.com/LDeakin/openzl-sys/releases/tag/v0.1.1+openzl.0.1.0
[0.1.0+openzl.0.1.0]: https://github.com/LDeakin/openzl-sys/releases/tag/v0.1.0+openzl.0.1.0
