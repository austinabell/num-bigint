//! Test serialization and deserialization of `BigUint` and `BigInt`
//!
//! The serialized formats should not change, even if we change our
//! internal representation, because we want to preserve forward and
//! backward compatibility of serialized data!

#![cfg(feature = "serde_derive")]

extern crate num_bigint;
extern crate num_traits;
extern crate serde_test;

use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde_test::{assert_tokens, Token};

#[test]
fn biguint_zero() {
    let tokens = [Token::Bytes(&[])];
    assert_tokens(&BigUint::zero(), &tokens);
}

#[test]
fn bigint_zero() {
    let tokens = [Token::Bytes(&[])];
    assert_tokens(&BigInt::zero(), &tokens);
}

#[test]
fn biguint_one() {
    let tokens = [Token::Bytes(&[0, 1])];
    assert_tokens(&BigUint::one(), &tokens);
}

#[test]
fn bigint_one() {
    let tokens = [Token::Bytes(&[0, 1])];
    assert_tokens(&BigInt::one(), &tokens);
}

#[test]
fn bigint_negone() {
    let tokens = [Token::Bytes(&[1, 1])];
    assert_tokens(&-BigInt::one(), &tokens);
}
