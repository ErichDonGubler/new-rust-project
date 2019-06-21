# New Rust Project

<!-- NOTE: You can also use a Crates.io version of this license badge: https://shields.io/category/license -->
[![License](https://img.shields.io/github/license/erichdongubler/new-rust-project.svg)](LICENSE.md)
[![Crates.io](https://img.shields.io/crates/v/new-rust-project.svg)](https://crates.io/crates/new-rust-project)
[![Docs.rs](https://docs.rs/new-rust-project/badge.svg)](https://docs.rs/new-rust-project)
[![Matrix chat](https://img.shields.io/matrix/new-rust-project:matrix.org.svg)](https://matrix.to/#/#new-rust-project:matrix.org)

[![last release date](https://img.shields.io/github/release-date/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/releases)
[![repo activity](https://img.shields.io/github/commit-activity/m/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/pulse/monthly)
[![contributors](https://img.shields.io/github/contributors/erichdongubler/new-rust-project.svg)](https://github.com/erichdongubler/new-rust-project/graphs/contributors)
[![Build Status](https://secure.travis-ci.org/erichdongubler/new-rust-project.svg?branch=master)](https://travis-ci.com/erichdongubler/new-rust-project)

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

This template uses [MPL 2.0](LICENSE.md) by default. Erich's reasons for MPL by default here are:

* Very permissive license in general
* Patent protection for that project that suddenly takes off
* Not-terribly-annoying copyleft

When in doubt, remember that Erich is not a lawyer. change your own project to use what you
deem appropriate.

### Contributing

Contributions, feature requests, and bug reports are warmly welcomed! See the [contribution
guidelines](CONTRIBUTING.md) for getting started.

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

### Intelligent defaults for docs

The [Rust Playground](https://play.rust-lang.org/) is used as the playground service by
default.

### `cargo-release` configuration

`cargo-release` is configured to keep features of this template in sync with version releases,
and has some defaults Erich considers more sensible.

### CHANGELOG

Yes, you should maintain a [`CHANGELOG`](CHANGELOG.md). ;)

### More aggressive linting and tests

Several `rustc` and `clippy` lints have been enabled that Erich prefers. See the top of
[`src/lib.rs`](src/lib.rs) for the full list.

Warnings are denied in doctests and in release mode.

### Out-of-the-box CI

The associated CI configuration (Travis at [`.travis.yml`](.travis.yml)) tests:
* Runs tests on Linux, Windows, and MacOS.
* The full set of lints with `cargo clippy`
* Formatting with `cargo fmt`
* The full suite of built-in tests with `cargo test`

### Badges!

There are a variety of handy badges on the top of the README. The first row of badges are
intended to aid crate users, while the second is intended for (maybe potential) contributors.
Some badges may not be suitable for, say, internal or private projects that won't actually be
published on `crates.io`. You are encouraged to keep the ones you want and throw out the rest.

If you're curious what other badges you can get out-of-the-box, check out
[Shields.io](https://shields.io).

<!-- cargo-sync-readme end -->
