// This file is part of Rust Crate Template <https://github.com/Fuwn/rust-crate-template>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

//! # Rust Crate Template
//!
//! [![crates.io](https://img.shields.io/crates/v/rust-crate-template.svg)](https://crates.io/crates/rust-crate-template)
//! [![docs.rs](https://docs.rs/rust-crate-template/badge.svg)](https://docs.rs/rust-crate-template)
//! [![github.com](https://github.com/Fuwn/rust-crate-template/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/Fuwn/rust-crate-template/actions/workflows/check.yaml)
//!
//! Rust Crate Template is a Rust crate template.
//!
//! ## Usage
//!
//! > Using this as a template? Try
//! > `cargo generate --git https://github.com/Fuwn/rust-crate-template` with
//! > [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)!
//!
//! ### Add Rust Crate Template as a dependency
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! rust-crate-template = "0.1.1"
//! ```
//!
//! ## Examples
//!
//! Examples can be found within the
//! [`examples/`](https://github.com/Fuwn/rust-crate-template/tree/main/examples) directory.
//!
//! ## License
//!
//! This project is licensed with the
//! [GNU General Public License v3.0](https://github.com/Fuwn/rust-crate-template/blob/main/LICENSE).

#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code,
  clippy::all,
  clippy::nursery,
  clippy::pedantic
)]
#![recursion_limit = "128"]

pub struct Adder<T> {
  a: T,
  b: T,
}
impl<T> Adder<T>
where T: std::ops::Add<Output = T>
{
  pub const fn new(a: T, b: T) -> Self {
    Self {
      a,
      b,
    }
  }

  pub fn add(self) -> T { self.a + self.b }
}
