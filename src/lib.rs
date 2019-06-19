/*
 * Copyright (c) 2019 `new-rust-project` contributors
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
//! This project is Erich's personal Rust starter kit for developing new libraries and binaries in
//! Rust. You shouldn't be seeing this anywhere outside of his
//! [`new-rust-project`](https://github.com/erichdongubler/new-rust-project) repo.
//!
//! ## Overview
//!
//! At one point, Erich got tired of accumulating lots of interesting tidbits for starting Rust
//! projects that he knew he'd forget. So he finally hunkered down and made this repo. An example
//! of usage:
//!
//! ```sh,ignore
//! #! /bin/sh
//!
//! git clone --shallow https://github.com/ErichDonGubler/new-rust-project name-of-new-rust-project
//! cd name-of-new-rust-project
//! rm -rf .git
//! git init
//! git add .
//! git commit -m "Initial commit"
//! git remote add origin git@github.com:ErichDonGubler/name-of-new-rust-project
//! git push -u origin master
//! ```
//!
//! ## Features
//!
//! ### Licensing
//!
//! This template uses [MPL 2.0]() by default. Erich's reasons for MPL by default here are:
//!
//! * Very permissive license in general
//! * Patent protection for that project that suddenly takes off
//! * Not-terribly-annoying copyleft
//!
//! When in doubt, remember that Erich is not a lawyer. change your own project to use what you
//! deem appropriate.
//!
//! ### Contributing
//!
//! Contributions, feature requests, and bug reports are warmly welcomed! See the [contribution
//! guidelines](CONTRIBUTING.md) for getting started.
//!
//! See also the [code of conduct](CODE_OF_CONDUCT.md) for more details about expectations
//! regarding contributions to this project.
//!
//! The [code of conduct](CODE_OF_CONDUCT.md) uses [Contributor Covenant
//! v1.4.1](https://www.contributor-covenant.org/version/1/4/code-of-conduct). If there's a newer
//! version of this, feel free to open a PR!
//!
//! ### Crate documentation in README
//!
//! Crate documentation is inlined into this README.  This means you get doc-tests for freebies!
//! Try it out by reading the README -- it uses `cargo-sync-readme`.  Also, this is integrated into
//! CI, so you don't forget about it!
//!
//! ```rust
//! println!("This should run just fine.");
//! ```
//!
//! ```rust,should_panic
//! panic!("This should panic.");
//! ```
//!
//! ```rust,compile_fail
//! !@#$% // This should fail to compile.
//! ```
//!
//! # CHANGELOG
//!
//! Yes, you should maintain a [`CHANGELOG`](CHANGELOG.md).
//!
//! ### More aggressive linting and tests
//!
//! Several `rustc` and `clippy` lints have been enabled that Erich prefers. See the top of
//! [`src/lib.rs`](src/lib.rs) for the full list.
//!
//! Warnings are denied in doctests and in release mode.
//!
//! ### Out-of-the-box CI
//!
//! The associated CI configuration (Travis at [`.travis.yml`](.travis.yml)) tests:
//! * Runs tests on Linux, Windows, and MacOS.
//! * The full set of lints with `cargo clippy`
//! * Formatting with `cargo fmt`
//! * The full suite of built-in tests with `cargo test`
//!
//! ### Buttons!
//!
//! There are a variety of handy buttons on the top of the README. These are meant to encourage
//! activity both for maintainers and newcomers. Some buttons may not be suitable for, say,
//! internal or private projects that won't actually be published on `crates.io`. You are
//! encouraged to keep the ones you want and throw out the rest.

#![cfg_attr(not(debug_assertions), deny(warnings))]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![doc(html_root_url = "https://docs.rs/new-rust-project/0.1")]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(warn(
    bare_trait_objects,
    clippy::cargo,
    clippy::pedantic,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    single_use_lifetimes,
    unused_extern_crates,
))))]
#![warn(
    bare_trait_objects,
    clippy::cargo,
    clippy::pedantic,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_docs,
    single_use_lifetimes,
    unused_extern_crates,
)]
#![allow(clippy::multiple_crate_versions)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
