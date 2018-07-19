// Copyright ⓒ 2015-2016 Kevin B. Knapp and clap-validators contributors.
// clap-validators is primarily distributed under the terms of both the MIT license and the Apache
// License (Version 2.0), with portions covered by various BSD-like licenses.
// See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.

//! # clap-validators
//!
//! [![Crates.io](https://img.shields.io/crates/v/clap-validators.svg)](https://crates.io/crates/clap-validators) [![Crates.io](https://img.shields.io/crates/d/clap-validators.svg)](https://crates.io/crates/clap-validators) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kbknapp/clap-validators/blob/master/LICENSE-MIT) [![Coverage Status](https://coveralls.io/repos/kbknapp/clap-validators/badge.svg?branch=master&service=github)](https://coveralls.io/github/kbknapp/clap-validators?branch=master) [![Join the chat at https://gitter.im/kbknapp/clap-validators](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/clap-validators?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
//!
//! Linux: [![Build Status](https://travis-ci.org/kbknapp/clap-validators.svg?branch=master)](https://travis-ci.org/kbknapp/clap-validators)
//! Windows: [![Build status](https://ci.appveyor.com/api/projects/status/ejg8c33dn31nhv36/branch/master?svg=true)](https://ci.appveyor.com/project/kbknapp/clap-validators/branch/master)
//!
//! Command Line Argument Parser for Rust
//!
//! It is a simple to use, efficient, and full featured library for parsing command line arguments and subcommands when writing console, or terminal applications.
//!
//! ## [documentation](http://kbknapp.github.io/clap-validators/clap-validators/index.html)
//!
//! Table of Contents
//! =================
//!
//! * [What's New](#whats-new)
//! * [About](#about)
//! * [Validators](#validators)
//! * [Quick Example](#quick-example)
//! * [Try it!](#try-it)
//!   * [BYOB (Build Your Own Binary)](#byob-build-your-own-binary)
//! * [Usage](#usage)
//!   * [More Information](#more-information)
//! * [How to Contribute](#how-to-contribute)
//!   * [Running the tests](#running-the-tests)
//! * [License](#license)
//! * [Deprecations](#deprecations)
//!
//! Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)
//!
//! ## What's New
//!
//! In v0.0.1
//!
//! #### New Features
//!
//!  * **Default Values**: Initial Release!
//!
//! For full details, see [CHANGELOG.md](https://github.com/kbknapp/clap-validators/blob/master/CHANGELOG.md)
//!
//! ## About
//!
//! `clap-validators` are pre-defined functions which can be dropped in to [clap](https://github.com/kbknapp/clap-rs) argument declarations to perform simple value validations.
//!
//! ## Validators
//!
//! Below are the validators which are included with `clap-validators`; full descriptions and usage can be found in the [documentation](http://kbknapp.github.io/clap-validators/clap-validators/index.html) and [examples/](examples) directory
//!
//! * `clap_validators::numeric`
//!  * `is_number` - Valid `i64` or `f64`
//!
//! ## Quick Example
//!
//! The following examples show a quick example of using `clap-validators` in combination with `clap` to perform some basic validation of arguments.
//!
//! ```rust ignore
//! extern crate clap;
//! extern crate clap_validators;
//!
//! use clap::{Arg, App};
//! use clap_validators;
//!
//! fn main() {
//!     let matches = App::new("My Super Program")
//!                           .arg(Arg::with_name("infile")
//!                                .short("i")
//!                                .value_name("FILE")
//!                                .help("Some file to read")
//!                                .takes_value(true)
//!                                .validator(clap_validators::fs::is_file))
//!                           .get_matches();
//!
//!     // program logic goes here, and we can make the assumption that "infile" is a valid file...
//! }
//! ```
//!
//! If you were to compile any of the above programs and run them with the flag `-i <file>` where `<file>` wasn't actually a valid file, one would see the graceful exit of:
//!
//! ```sh
//! $ myprog -i not-a-file
//! error: not-a-file isn't a valid file
//! ```
//!
//! ## Try it!
//!
//! To test out `clap-validators`:
//! * Create a new cargo project `$ cargo new fake --bin && cd fake`
//! * Add `clap-validators` and `clap` to your `Cargo.toml`
//! *
//! ```toml
//! [dependencies]
//! clap = "2"
//! clap-validators = "0"
//! ```
//!
//! * Add the following to your `src/main.rs`
//!
//! ```rust ignore
//! extern crate clap;
//! extern crate clap_validators;
//!
//! use clap::{App, Arg};
//! use clap_validators;
//!
//! fn main() {
//!   let m = App::new("fake")
//!     .arg(Arg::with_name("infile")
//!         .required(true)
//!         .validator(clap_validators::fs::is_file))
//!     .get_matches();
//!
//!     println!("success!");
//! }
//! ```
//!
//! * Build your program `$ cargo build --release`
//! * Create a file, since we'll be testing if something is a file or not `touch myfile`
//! * Run with help or version `$ ./target/release/fake myfile`
//!  * Or to see this fail, run again with something that is not a file, such as a directory `mkdir mydir`, `./target/release/fake mydir`
//!
//! ## Usage
//!
//! For full usage, add `clap-validators` as a dependency in your `Cargo.toml` file to use from crates.io:
//!
//!  ```toml
//!  [dependencies]
//!  clap_validators = "0"
//!  ```
//!
//!  Or track the latest on the master branch at github:
//!
//! ```toml
//! [dependencies.clap_validators]
//! git = "https://github.com/kbknapp/clap-validators.git"
//! ```
//!
//! Add `extern crate clap_validators;` to your crate root.
//!
//! ### More Information
//!
//! You can find complete documentation on the [github-pages site](http://kbknapp.github.io/clap-validators/clap_validators/index.html) for this project.
//!
//! You can also find usage examples in the [examples/](examples) directory of this repo.
//!
//! ## How to Contribute
//!
//! Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from adding validators, documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!
//!
//! Another really great way to help is if you find an interesting, or helpful way in which to use `clap-validators`. You can either add it to the [examples/](examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)
//!
//! Please read [CONTRIBUTING.md](.github/CONTRIBUTING.md) before you start contributing.
//!
//! ### Running the tests
//!
//! If contributing, you can run the tests as follows (assuming you're in the `clap-validators` directory)
//!
//! ```ignore
//! $ cargo test
//!
//! # Only on nightly compiler:
//! $ cargo build --features lints
//! ```
//!
//! ## License
//!
//! `clap-validators` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.
//!
//! See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
//!
//! ### Deprecations
//!
//! Old method names will be left around for several minor version bumps, or one major version bump.
//!
//! As of 0.0.1:
//!
//!  * None!

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny(unstable_features))]
#![deny(
        missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

/// Validators which work on the file system, such as determining valid files, links, or
/// directories
pub mod fs;
/// Validators which work on numbers
pub mod num;
