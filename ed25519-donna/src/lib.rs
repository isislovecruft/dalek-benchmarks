// -*- mode: rust; coding: utf-8; -*-
//
// Copyright (c) 2017 Isis Agora Lovecruft
//
// See LICENCE for licensing information.

#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(test, feature = "bench"))]
extern crate test;

#[cfg(all(test, feature = "bench"))]
extern crate libc;

#[cfg(all(test, feature = "bench"))]
pub mod ffi;

#[cfg(all(test, feature = "bench"))]
mod bench;
