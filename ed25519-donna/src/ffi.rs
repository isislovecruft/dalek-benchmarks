// -*- mode: rust; coding: utf-8; -*-
//
// Copyright (c) 2017 Isis Agora Lovecruft
//
// See LICENCE for licensing information.

use libc::{c_int, c_uchar, c_void, size_t};

// For benchmarking group operations with curve25519-dalek versus
// ed25519-donna's "curved25519" function.
#[cfg(feature = "groupcmp")]
use curve25519_dalek::edwards::CompressedEdwardsY;
#[cfg(feature = "groupcmp")]
use curve25519_dalek::edwards::ExtendedPoint;

#[link(name = "ed25519donna")]
extern "C" {
    fn ed25519_publickey(sk: *const c_uchar, pk: *mut c_uchar);
    fn ed25519_sign(m: *const c_uchar, mlen: size_t, sk: *const c_uchar, pk: *const c_uchar, RS: *mut c_uchar);
    fn ed25519_sign_open(m: *const c_uchar, mlen: size_t, pk: *const c_uchar, RS: *const c_uchar) -> c_int;
    fn ed25519_sign_open_batch(m: *const *const c_uchar, mlen: *const size_t, pk: *const *const c_uchar, RS: *const *const c_uchar, num: size_t, valid: *mut c_int) -> c_int;
    fn ed25519_randombytes_unsafe(out: *mut c_void, count: size_t);
    fn curved25519_scalarmult_basepoint(pk: *mut c_uchar, e: *const c_uchar);
}

pub fn ed25519_donna_publickey(secret_key: &[u8; 32], public_key: &mut [u8; 32]) {
    unsafe {
        ed25519_publickey(secret_key.as_ptr(), public_key.as_mut_ptr());
    }
}

pub fn ed25519_donna_sign(message: &[u8], secret_key: &[u8; 32], public_key: &[u8; 32]) -> [u8; 64] {
    let mut signature: [u8; 64] = [0u8; 64];
    let message_len: size_t = message.len();

    unsafe {
        ed25519_sign(message.as_ptr(), message_len, secret_key.as_ptr(), public_key.as_ptr(), signature.as_mut_ptr());
    }

    signature
}

pub fn ed25519_donna_sign_open(message: &[u8], public_key: &[u8; 32], signature: &[u8; 64]) -> i32 {
    let message_len: size_t = message.len();

    unsafe {
        ed25519_sign_open(message.as_ptr(), message_len, public_key.as_ptr(), signature.as_ptr())
    }
}

#[cfg(feature = "groupcmp")]
pub fn ed25519_donna_curved25519_scalarmult_basepoint(scalar: &[u8; 32]) -> CompressedEdwardsY {
    let mut output: [u8; 32] = [0u8; 32];

    unsafe {
        curved25519_scalarmult_basepoint(output.as_mut_ptr(), scalar.as_ptr());
    }

    CompressedEdwardsY(output)
}
