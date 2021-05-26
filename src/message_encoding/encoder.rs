use ark_ec::bn::{BnParameters, G1Affine};
use ark_ec::SWModelParameters;
use ark_ff::{Field, FpParameters, One, PrimeField, SquareRootField, Zero, LegendreSymbol};
use ark_std::cmp::Ordering;
use ark_std::ops::{Add, Div, Shr};
use ark_std::rand::RngCore;
use ark_std::{marker::PhantomData, ops::Neg, vec::Vec, UniformRand};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::ToPrimitive;

#[derive(Default)]
pub struct Encoder<P: BnParameters> {
    pub q: BigUint,
    pub minus_one: P::Fp,
    pub sqrt_minus_3: P::Fp,
    pub q_minus_1_div_2: P::Fp,
    pub b: P::Fp,
    pub b_plus_one: P::Fp,
    pub sqrt_minus_3_minus_1_div_2: P::Fp,
    pub minus_sqrt_minus_3_div_2: P::Fp,
    pub legendre_2: i32,

    /// [u64] representation of (q + 1) / 4.
    /// Powering a quadratic residue by this, we can obtain one of the square root, for q == 3 (mod 4).
    pub square_root_pow: Vec<u64>,

    #[doc(hidden)]
    pub phantom: PhantomData<P>,
}

impl<P: BnParameters> Encoder<P> {
    pub fn new() -> Self {
        let minus_3: P::Fp = <P as BnParameters>::Fp::from(3u64).neg();
        let sqrt_minus_3 = minus_3.sqrt().unwrap();

        let q: BigUint = <<P as BnParameters>::Fp as PrimeField>::Params::MODULUS.into();
        let q_minus_1_div_2 = P::Fp::from_repr(
            <<P as BnParameters>::Fp as PrimeField>::Params::MODULUS_MINUS_ONE_DIV_TWO,
        )
        .unwrap();

        let b = P::G1Parameters::COEFF_B;
        let b_plus_one = b + &<P as BnParameters>::Fp::one();

        let minus_one = <P as BnParameters>::Fp::one().neg();

        let sqrt_minus_3_minus_1_div_2 =
            (sqrt_minus_3 + &minus_one) * &<P as BnParameters>::Fp::from(2u64).inverse().unwrap();

        let minus_sqrt_minus_3_div_2 =
            sqrt_minus_3.neg() * <P as BnParameters>::Fp::from(2u64).inverse().unwrap();

        let legendre_2 = match <P as BnParameters>::Fp::from(2u64).legendre() {
            LegendreSymbol::Zero => 0,
            LegendreSymbol::QuadraticResidue => 1,
            LegendreSymbol::QuadraticNonResidue => -1
        };

        let square_root_pow = {
            let tmp: BigUint = q.clone().add(1u64).div(4u64);
            let bytes = tmp.to_bytes_le();

            let mut limbs = Vec::new();

            bytes.chunks(8).into_iter().for_each(|chunk| {
                let mut chunk_padded = [0u8; 8];
                chunk_padded[..chunk.len()].copy_from_slice(chunk);
                limbs.push(u64::from_le_bytes(chunk_padded))
            });

            limbs
        };

        Self {
            q,
            minus_one,
            sqrt_minus_3,
            q_minus_1_div_2,
            b,
            b_plus_one,
            sqrt_minus_3_minus_1_div_2,
            minus_sqrt_minus_3_div_2,
            legendre_2,
            square_root_pow,
            phantom: PhantomData,
        }
    }

    pub fn encode<R: RngCore>(&self, val: P::Fp, rng: &mut R) -> G1Affine<P> {
        // The following algorithm from [FT10] is equivalent to the original formulas for x_1, x_2, x_3
        // [FT10]: Pierre-Alain Fouque and Mehdi Tibouchi, "Indifferentiable Hashing to Barreto–Naehrig Curves", LATINCRYPT 2012.

        // val is v on the paper
        // w is 1/u; w.inverse() = u

        let w = self.sqrt_minus_3 * &val * &(self.b_plus_one + &val.square()).inverse().unwrap();

        let x1 = self.sqrt_minus_3_minus_1_div_2 - val * &w;
        let x2 = self.minus_one - &x1;
        let x3 = <P as BnParameters>::Fp::one()
            + <P as BnParameters>::Fp::one() * &w.square().inverse().unwrap();

        // Sample r1 and r2 for data-independent-time masking
        let r1 = P::Fp::rand(rng);
        let r2 = P::Fp::rand(rng);

        // Compute the Legendre symbols of (r1 ^ 2 * (x1 ^ 3 + b)) and (r2 ^ 2 * (x2 ^ 3 + b))
        let alpha = self.compute_legendre_symbol(r1.square() * &(x1.square() * &x1 + &self.b));
        let beta = self.compute_legendre_symbol(r2.square() * &(x2.square() * &x2 + &self.b));

        let idx = ((alpha - 1) * beta % 3 + 1) as u8;

        let x = {
            let mut x = x1;

            let mut sel = idx.eq(&2u8) as u8;
            x = x * &P::Fp::from((1 - sel) as u128) + x2 * &P::Fp::from(sel as u128);

            sel = idx.eq(&3u8) as u8;
            x = x * &P::Fp::from((1 - sel) as u128) + x3 * &P::Fp::from(sel as u128);

            x
        };

        let y = x * &x.square() + &self.b;

        // Compute the special character
        let character = self.compute_character(idx, val, w.inverse().unwrap());

        // Output the point
        let y = self.compute_square_root(y) * character;
        let point = G1Affine::<P>::new(x.clone(), y, false);
        assert!(point.is_on_curve());

        point
    }

    pub fn compute_character(&self, idx: u8, val: P::Fp, u: P::Fp) -> P::Fp {
        let one = <P as BnParameters>::Fp::one();
        let minus_one = self.minus_one;

        if idx == 1 || idx == 2 {
            // CASE 1
            if val.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                one
            } else {
                minus_one
            }
        } else {
            // CASE 2
            let comp_number = u * self.minus_sqrt_minus_3_div_2;
            if val.cmp(&comp_number) == Ordering::Less {
                one
            } else {
                minus_one
            }
        }
    }

    #[inline]
    pub fn compute_legendre_symbol(&self, val: P::Fp) -> i32 {
        // Compute the Legendre symbol via the law of quadratic reciprocity (in the Jacobi case).
        assert!(!val.is_zero());

        let mut p: BigUint = val.into();
        let mut q = self.q.clone();
        let mut cur = 1;

        while p.is_even() {
            p = p.shr(1);
            cur *= self.legendre_2;
        }

        while !p.is_one() {
            let mut new_p = q.clone() % p.clone();
            let new_q = p.clone();

            let mut adjustment = -1;

            if (p.clone() % BigUint::from(4u64)).is_one() {
                adjustment = 1;
            }

            if (q.clone() % BigUint::from(4u64)).is_one() {
                adjustment = 1;
            }

            cur *= adjustment;

            let legendre_2_cur = {
                let tmp = (new_q.clone() % BigUint::from(8u64)).to_u8().unwrap();
                if tmp == 1 || tmp == 7 {
                    1
                } else {
                    -1
                }
            };

            while new_p.is_even() {
                new_p = new_p.shr(1);
                cur *= legendre_2_cur;
            }

            p = new_p;
            q = new_q;
        }

        cur
    }

    #[inline]
    pub fn compute_square_root(&self, val: P::Fp) -> P::Fp {
        val.pow(&self.square_root_pow)
    }
}

#[cfg(test)]
mod test {
    use crate::curve_bn446::Parameters as Bn446Parameters;
    use crate::message_encoding::encoder::Encoder;
    use ark_ec::bn::BnParameters;
    use ark_std::UniformRand;
    use ark_ff::{SquareRootField, LegendreSymbol};

    const REPETITIONS: u64 = 10;

    #[test]
    fn test_precomputation() {
        let _encoder = Encoder::<Bn446Parameters>::new();

        unimplemented!();
    }

    #[test]
    fn test_square_root() {
        unimplemented!();
    }

    #[test]
    fn test_legendre_symbol() {
        let mut rng = ark_std::test_rng();
        let encoder = Encoder::<Bn446Parameters>::new();

        for _ in 0..REPETITIONS {
            let a = <Bn446Parameters as BnParameters>::Fp::rand(&mut rng);

            let res_encoder = encoder.compute_legendre_symbol(a);

            let res_standard = match a.legendre() {
                LegendreSymbol::Zero => 0,
                LegendreSymbol::QuadraticResidue => 1,
                LegendreSymbol::QuadraticNonResidue => -1
            };

            assert_eq!(res_encoder, res_standard);
        }
    }

    // TODO: see what other tests are needed
    // TODO: fix the Legendre symbol, find an algorithm that is faster than simply doing exp
}