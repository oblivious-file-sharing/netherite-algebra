#![allow(unused_imports)]
use ark_ec::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{
    field_new,
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    One, Zero,
};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::Rng;
use ark_std::test_rng;
use core::ops::{AddAssign, MulAssign};

use crate::curve_bn446::{
    g1, g2, Bn446, Fq, Fq12, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective,
};

use ark_algebra_test_templates::{curves::*, groups::*};
use ark_ec::bn::BnParameters;
use ark_std::ops::{Add, Mul, Shl};

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();

    sw_tests::<g1::Parameters>();
}

#[test]
fn test_g1_projective_group() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G1Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_projective_curve() {
    curve_tests::<G2Projective>();

    sw_tests::<g2::Parameters>();
}

#[test]
fn test_g2_projective_group() {
    let mut rng = test_rng();
    let a: G2Projective = rng.gen();
    let b: G2Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}
