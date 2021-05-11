use ark_ec::models::{ModelParameters, SWModelParameters};
use ark_ff::{field_new, Zero};

use crate::curve_bn254::{Fq, Fq2, Fr};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = [0, 0]
    #[rustfmt::skip]
    const COEFF_A: Fq2 = field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"));

    /// COEFF_B = 2 / xi
    ///         = -u + 1
    /// where xi = (u + 1) is an element in Fq2 is not square (i.e., quadratic nonresidue) and is not a cube
    #[rustfmt::skip]
    const COEFF_B: Fq2 = field_new!(Fq2,
        field_new!(Fq, "1"),
        field_new!(Fq, "-1"),
    );

    /// COFACTOR = (36 * X^4) + (36 * X^3) + (30 * X^2) + 6*X + 1
    ///          = 16798108731015832284940804142231733910018794639473386948839051987979937513497
    /// For this curve, X = -(2^62 + 2^55 + 1) = -4647714815446351873
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        12465963768561532953u64,
        14024912927073501206u64,
        13417434401994702856u64,
        2676093114170474497u64,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "8399054365507916140663265725831889489001708940320597534943659766172318957567");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

#[rustfmt::skip]
pub const G2_GENERATOR_X: Fq2 = field_new!(Fq2, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
#[rustfmt::skip]
pub const G2_GENERATOR_Y: Fq2 = field_new!(Fq2, G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 12723517038133731887338407189719511622662176727675373276651903807414909099441
#[rustfmt::skip]
pub const G2_GENERATOR_X_C0: Fq = field_new!(Fq, "12723517038133731887338407189719511622662176727675373276651903807414909099441");

/// G2_GENERATOR_X_C1 =
/// 4168783608814932154536427934509895782246573715297911553964171371032945126671
#[rustfmt::skip]
pub const G2_GENERATOR_X_C1: Fq = field_new!(Fq, "4168783608814932154536427934509895782246573715297911553964171371032945126671");

/// G2_GENERATOR_Y_C0 =
/// 13891744915211034074451795021214165905772212241412891944830863846330766296736
#[rustfmt::skip]
pub const G2_GENERATOR_Y_C0: Fq = field_new!(Fq, "13891744915211034074451795021214165905772212241412891944830863846330766296736");

/// G2_GENERATOR_Y_C1 =
/// 7937318970632701341203597196594272556916396164729705624521405069090520231616
#[rustfmt::skip]
pub const G2_GENERATOR_Y_C1: Fq = field_new!(Fq, "7937318970632701341203597196594272556916396164729705624521405069090520231616");

// We use the G2 generator in https://github.com/herumi/ate-pairing/blob/530223d7502e95f6141be19addf1e24d27a14d50/test/test_point.hpp
