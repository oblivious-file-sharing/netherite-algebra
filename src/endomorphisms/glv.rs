use ark_ec::bn::{BnParameters, G1Affine};
use ark_ec::{AffineCurve, ModelParameters, ProjectiveCurve};
use ark_ff::{BigInteger, FpParameters, PrimeField};
use ark_std::rand::Rng;

pub trait BnGlvParameters: BnParameters {
    const BETA: <Self as BnParameters>::Fp;
    const LAMBDA: <Self::G1Parameters as ModelParameters>::ScalarField;
}

pub fn glv_rand_gen<P: BnGlvParameters, R: Rng>(
    len: usize,
    rng: &mut R,
) -> Vec<(
    <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::BigInt,
    <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::BigInt,
)> {
    // This does not guarantee that the values would be uniformly random in the F_p
    // However, for the purpose of random linear combination, this is sufficient.

    let num_bits =
        <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::Params::MODULUS_BITS / 2;

    let mut res = Vec::with_capacity(len);

    let mut left_bits = vec![false; num_bits as usize];
    let mut right_bits = vec![false; num_bits as usize];

    for i in 0..len {
        rng.fill(left_bits.as_mut_slice());
        rng.fill(right_bits.as_mut_slice());

        let left_bigint =
            <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::BigInt::from_bits_le(
                &left_bits,
            );
        let right_bigint =
            <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::BigInt::from_bits_le(
                &right_bits,
            );

        res.push((left_bigint, right_bigint));
    }

    res
}

pub fn glv_mul<P: BnGlvParameters>(
    val: &G1Affine<P>,
    rand: &(
        <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::BigInt,
        <<P::G1Parameters as ModelParameters>::ScalarField as PrimeField>::BigInt,
    ),
) -> G1Affine<P> {
    let res_1 = val.into_projective().mul(rand.0);
    let res_2 = glv_apply::<P>(val).into_projective().mul(rand.1);

    (res_1 + res_2).into_affine()
}

pub fn glv_apply<P: BnGlvParameters>(val: &G1Affine<P>) -> G1Affine<P> {
    if val.infinity {
        (*val).clone()
    } else {
        let x_new = val.x * P::BETA;
        G1Affine::<P>::new(x_new, val.y, val.infinity)
    }
}
