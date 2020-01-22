//! These docs are really for `new-rust-project` maintainers. If you're curious how to use the
//! built binary, refer to `new-rust-project --help` for more details.

#![cfg_attr(not(debug_assertions), deny(warnings))]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(warn(
    bare_trait_objects,
    clippy::cargo,
    clippy::pedantic,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
))))]
#![warn(
    bare_trait_objects,
    clippy::cargo,
    clippy::pedantic,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]
#![allow(
    clippy::default_trait_access,
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions
)]

#[allow(missing_docs)]
pub fn main() {

}
