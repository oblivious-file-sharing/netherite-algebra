use crate::curve_bn254::*;
use ark_ec::{
    bn,
    bn::{Bn, BnParameters, TwistType},
};
use ark_ff::field_new;
pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub struct Parameters;

impl BnParameters for Parameters {
    const X: &'static [u64] = &[4647714815446351873];
    /// `x` is negative.
    const X_IS_NEGATIVE: bool = true;

    const ATE_LOOP_COUNT: &'static [i8] = &[
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0,
        0, 0, 0, 1, 1,
    ];

    /// Point Q x coordinate
    /// Recall that xi = u + 1, this is xi ^ ((q - 1) / 3)
    const TWIST_MUL_BY_Q_X: Fq2 = field_new!(
        Fq2,
        FQ_ZERO,
        field_new!(
            Fq,
            "16798108731015832283133667796947756444075910019074449559301910896669540483083"
        ),
    );
    // Point Q y coordinate
    /// Recall that xi = u + 1, this is xi ^ ((q - 1) / 2)
    const TWIST_MUL_BY_Q_Y: Fq2 = field_new!(
        Fq2,
        field_new!(
            Fq,
            "16226349498735898878582721725794281106152147739300925444201528929117996286405"
        ),
        field_new!(
            Fq,
            "16226349498735898878582721725794281106152147739300925444201528929117996286405"
        ),
    );
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Params = Fq2Parameters;
    type Fp6Params = Fq6Parameters;
    type Fp12Params = Fq12Parameters;
    type G1Parameters = g1::Parameters;
    type G2Parameters = g2::Parameters;
}

pub type Bn254 = Bn<Parameters>;

pub type G1Affine = bn::G1Affine<Parameters>;
pub type G1Projective = bn::G1Projective<Parameters>;
pub type G2Affine = bn::G2Affine<Parameters>;
pub type G2Projective = bn::G2Projective<Parameters>;
