# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Bump MSRV to 1.77
- Bump `openzl` to 0.2.0
  - **Breaking**: `ZL_CompressIntrospectionHooks_s` fields added / reordered
  - **Breaking**: `ZL_HAVE_FBCODE` replaced with `ZL_IS_FBCODE`
  - **Breaking**: `ZL_Data_s` replaced with `Stream_s`
  - **Breaking**: `ZL_NodeParameters`, `ZL_ParameterizedNodeDesc`, `ZL_SelectorDesc`, and `ZL_MIEncoderDesc` gained dictionary, materializer, and MParam fields
  - **Breaking**: `ZL_E_addFrame_public()` replaced by `ZL_E_addFrame()`
  - **Breaking**: `ZL_Compressor_cloneNode()` removed
  - **Breaking**: `ZL_PipeDstCapacityFn` split into `ZL_CPipeDstCapacityFn` and `ZL_DPipeDstCapacityFn`
  - **Breaking**: `ZL_reportError()` and `ZL_ENABLE_RET_IF_ARG_PRINTING` removed
  - Add `ZL_Comment`, `ZL_Segmenter_getOperationContext()`, `ZL_Materializer_getOperationContext()`, `ZL_RuntimeGraphParameters`, `ZL_CCtx_addHeaderComment()`, and `ZL_FrameInfo_getComment()`
  - Add dictionary and materialization bindings, including `ZL_UniqueID`, `ZL_DictID`, `ZL_MParamID`, `ZL_BundleID`, `ZL_Materializer`, `ZL_DictLoader`, `ZL_MaterializerDesc`, `ZL_MaterializerDesc2`, `ZL_MParam`, `ZL_Compressor_loadDictBundle()`, and node dictionary/MParam query helpers
  - Add decompression introspection hook bindings and `ZL_DParam_enableCodecFusion`
  - Add graph-depth, sentinel-node, LZ4, and untrained ML selector helper bindings
  - Add additional `ZL_StandardGraphID_*`, `ZL_StandardNodeID_*`, local parameter ID, error code, format/chunk overhead, and minimum chunk size constants

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

[unreleased]: https://github.com/LDeakin/openzl-sys/compare/v0.1.2+openzl.0.1.0...HEAD
[0.1.2+openzl.0.1.0]: https://github.com/LDeakin/openzl-sys/releases/tag/v0.1.2+openzl.0.1.0
[0.1.1+openzl.0.1.0]: https://github.com/LDeakin/openzl-sys/releases/tag/v0.1.1+openzl.0.1.0
[0.1.0+openzl.0.1.0]: https://github.com/LDeakin/openzl-sys/releases/tag/v0.1.0+openzl.0.1.0
