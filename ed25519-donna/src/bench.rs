// -*- mode: rust; coding: utf-8; -*-
//
// Copyright (c) 2017 Isis Agora Lovecruft
//
// See LICENCE for licensing information.

/// Benchmarks from Rust of calling various C functions in ed25519-donna.
///
/// We assume these benchmarks are a mostly fair comparison to the ed25519-dalek
/// benchmarks, since the overhead of a single function call is negligible
/// compared to the operations being benched.

use ffi::ed25519_donna_publickey;
use ffi::ed25519_donna_sign;
use ffi::ed25519_donna_sign_open;
use ffi::ed25519_donna_curved25519_scalarmult_basepoint;

use test::Bencher;

const MESSAGE: &'static str = "This is a test of the tsunumi alert system. This is just a test.";

const ED25519_SECRET_KEY_BYTES: [u8; 32] = [
    157, 097, 177, 157, 239, 253, 090, 096,
    186, 132, 074, 244, 146, 236, 044, 196,
    068, 073, 197, 105, 123, 050, 105, 025,
    112, 059, 172, 003, 028, 174, 127, 096, ];

const ED25519_PUBLIC_KEY_BYTES: [u8; 32] = [
    215, 090, 152, 001, 130, 177, 010, 183,
    213, 075, 254, 211, 201, 100, 007, 058,
    014, 225, 114, 243, 218, 166, 035, 037,
    175, 002, 026, 104, 247, 007, 081, 026, ];

#[bench]
fn bench_ed25519_donna_publickey(b: &mut Bencher) {
    let mut public_key: [u8; 32] = [0u8; 32];

    b.iter(|| ed25519_donna_publickey(&ED25519_SECRET_KEY_BYTES, &mut public_key));
}

#[bench]
fn bench_ed25519_donna_sign(b: &mut Bencher) {
    b.iter(|| ed25519_donna_sign(&MESSAGE.as_bytes(), &ED25519_SECRET_KEY_BYTES, &ED25519_PUBLIC_KEY_BYTES));
}

#[bench]
fn bench_ed25519_donna_sign_open(b: &mut Bencher) {
    let signature: [u8; 64] = ed25519_donna_sign(&MESSAGE.as_bytes(), &ED25519_SECRET_KEY_BYTES, &ED25519_PUBLIC_KEY_BYTES);

    b.iter(|| ed25519_donna_sign_open(&MESSAGE.as_bytes(), &ED25519_PUBLIC_KEY_BYTES, &signature));
}

#[bench]
fn bench_ed25519_donna_scalarmult_basepoint(b: &mut Bencher) {
    let scalar: [u8; 32] = [  20, 130, 129, 196, 247, 182, 211, 102,
                              11, 168, 169, 131, 159,  69, 126,  35,
                             109, 193, 175,  54, 118, 234, 138,  81,
                              60, 183,  80, 186,  92, 248, 132,  13, ];

    b.iter(|| ed25519_donna_curved25519_scalarmult_basepoint(&scalar));
}
