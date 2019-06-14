# New Rust Project

[![Build Status](https://secure.travis-ci.org/erichdongubler/new-rust-project.svg?branch=master)](https://travis-ci.org/erichdongubler/new-rust-project)
<!-- NOTE: You can also use a Crates.io version of this license badge: https://shields.io/category/license -->
[![License](https://raw.githubusercontent.com/erichdongubler/new-rust-project/master/LICENSE)](LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/new-rust-project.svg)](https://crates.io/crates/new-rust-project)
[![Docs.rs](https://docs.rs/new-rust-project/badge.svg)](https://docs.rs/new-rust-project)
[![last release date](https://img.shields.io/github/release-date/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/releases)
[![repo activity](https://img.shields.io/github/commit-activity/m/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/pulse/monthly)
[![last commit](https://img.shields.io/github/last-commit/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/commits)
[![contributors](https://img.shields.io/github/contributors/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/graphs/contributors)

<!-- cargo-sync-readme start -->

This project is Erich's personal Rust starter kit for developing new libraries and binaries in
Rust. You shouldn't be seeing this anywhere outside of his
[`new-rust-project`](https://github.com/erichdongubler/new-rust-project) repo.

## Overview

At one point, Erich got tired of accumulating lots of interesting tidbits for starting Rust
projects that he knew he'd forget. So he finally hunkered down and made this repo. An example
of usage:

```sh,ignore
#! /bin/sh

git clone --shallow https://github.com/ErichDonGubler/new-rust-project name-of-new-rust-project
cd name-of-new-rust-project
rm -rf .git
git init
git add .
git commit -m "Initial commit"
git remote add origin git@github.com:ErichDonGubler/name-of-new-rust-project
git push -u origin master
```

## Features

### Licensing

This template uses [MPL 2.0]() by default. Erich's reasons for MPL by default here are:

* Very permissive license in general
* Patent protection for that project that suddenly takes off
* Not-terribly-annoying copyleft

When in doubt, remember that Erich is not a lawyer. change your own project to use what you
deem appropriate.

### Contributing

Contributions, feature requests, and bug reports are warmly welcomed! See the [contribution
guidelines](CONTRIBUTING.md) for getting started.

See also the [code of conduct](CODE_OF_CONDUCT.md) for more details about expectations
regarding contributions to this project.

The [code of conduct](CODE_OF_CONDUCT.md) uses [Contributor Covenant
v1.4.1](https://www.contributor-covenant.org/version/1/4/code-of-conduct). If there's a newer
version of this, feel free to open a PR!

### Crate documentation in README

Crate documentation is inlined into this README.  This means you get doc-tests for freebies!
Try it out by reading the README -- it uses `cargo-sync-readme`.  Also, this is integrated into
CI, so you don't forget about it!

```rust
println!("This should run just fine.");
```

```rust,should_panic
panic!("This should panic.");
```

```rust,compile_fail
!@#$% // This should fail to compile.
```

# CHANGELOG

Yes, you should maintain a [`CHANGELOG`](CHANGELOG.md).

### More aggressive linting

Several `rustc` and `clippy` lints have been enabled that Erich prefers. See the top of
[`src/lib.rs`](src/lib.rs) for the full list.

### Out-of-the-box CI

The associated CI configuration (Travis at [`.travis.yml`](.travis.yml)) tests:
* Runs tests on Linux, Windows, and MacOS.
* The full set of lints with `cargo clippy`
* Formatting with `cargo fmt`
* The full suite of built-in tests with `cargo test`

### Buttons!

There are a variety of handy buttons on the top of the README. These are meant to encourage
activity both for maintainers and newcomers. Some buttons may not be suitable for, say,
internal or private projects that won't actually be published on `crates.io`. You are
encouraged to keep the ones you want and throw out the rest.

<!-- cargo-sync-readme end -->
