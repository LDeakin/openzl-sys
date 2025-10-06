//! # openzl-sys
//!
//! Raw Rust bindings to OpenZL (<https://github.com/facebook/openzl>), a novel data compression framework.
//!
//! ## Bindings
//! This library includes a pre-generated `bindings.rs` file for `openzl.h`. New bindings can be generated using the bindgen feature:
//! ```bash
//! cargo build --features bindgen
//! ```
//!
//! ## Licence
//! `openzl-sys` is licensed under either of
//!  - the Apache License, Version 2.0 [LICENSE-APACHE](./LICENCE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0> or
//!  - the MIT license [LICENSE-MIT](./LICENCE-MIT) or <http://opensource.org/licenses/MIT>, at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(rustdoc::broken_intra_doc_links)]

extern crate zstd_sys;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));
