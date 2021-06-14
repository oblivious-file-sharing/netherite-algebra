use ark_ec::bn::{BnParameters, G1Affine};
use ark_ec::SWModelParameters;
use ark_ff::{Field, FpParameters, LegendreSymbol, One, PrimeField, SquareRootField, Zero};
use ark_std::cmp::Ordering;
use ark_std::mem::MaybeUninit;
use ark_std::ops::{Add, Div, BitAnd};
use ark_std::rand::RngCore;
use ark_std::{marker::PhantomData, ops::Neg, vec::Vec, UniformRand};
use gmp_mpfr_sys::gmp;
use num_bigint::BigUint;
use subtle::{ConstantTimeEq, ConditionallySelectable};

pub mod hybrid;

pub type DecodeHint = u8;

pub struct Encoder<P: BnParameters> {
    pub q: gmp::mpz_t,

    pub b: P::Fp,
    pub b_plus_one: P::Fp,

    pub minus_one: P::Fp,
    pub minus_3: P::Fp,
    pub minus_4_times_b_plus_1: P::Fp,

    pub inv_2: P::Fp,
    pub legendre_2: i32,

    pub sqrt_minus_3: P::Fp,
    pub minus_sqrt_minus_3: P::Fp,
    pub sqrt_minus_3_minus_1_div_2: P::Fp,
    pub minus_sqrt_minus_3_div_2: P::Fp,

    pub q_minus_1_div_2: P::Fp,

    /// [u64] representation of (q + 1) / 4.
    /// Powering a quadratic residue by this, we can obtain one of the square root, for q == 3 (mod 4).
    pub square_root_pow: Vec<u64>,

    #[doc(hidden)]
    pub phantom: PhantomData<P>,
}

unsafe impl<P: BnParameters> Sync for Encoder<P> {}

impl<P: BnParameters> Encoder<P> {
    pub fn new() -> Self {
        let q = unsafe {
            let mut q = MaybeUninit::uninit();
            gmp::mpz_init_set_ui(q.as_mut_ptr(), 0u64);

            let mut q = q.assume_init();

            let repr = <<P as BnParameters>::Fp as PrimeField>::Params::MODULUS;
            let limbs: &[u64] = AsRef::<[u64]>::as_ref(&repr);
            for limb in limbs.iter().rev() {
                gmp::mpz_mul_2exp(&mut q, &q, 64);
                gmp::mpz_add_ui(&mut q, &q, *limb);
            }

            q
        };

        let b = P::G1Parameters::COEFF_B;
        let b_plus_one = b + &<P as BnParameters>::Fp::one();

        let minus_one = <P as BnParameters>::Fp::one().neg();
        let minus_3: P::Fp = <P as BnParameters>::Fp::from(3u64).neg();
        let minus_4_times_b_plus_1 = <P as BnParameters>::Fp::from(4u64).neg() * &b_plus_one;

        let inv_2 = <P as BnParameters>::Fp::from(2u64).inverse().unwrap();
        let legendre_2 = match <P as BnParameters>::Fp::from(2u64).legendre() {
            LegendreSymbol::Zero => 0,
            LegendreSymbol::QuadraticResidue => 1,
            LegendreSymbol::QuadraticNonResidue => -1,
        };

        let sqrt_minus_3 = minus_3.sqrt().unwrap();
        let minus_sqrt_minus_3 = sqrt_minus_3.neg();
        let sqrt_minus_3_minus_1_div_2 = (minus_one + &sqrt_minus_3) * &inv_2;
        let minus_sqrt_minus_3_div_2 = inv_2 * &minus_sqrt_minus_3;

        let q_minus_1_div_2 = P::Fp::from_repr(
            <<P as BnParameters>::Fp as PrimeField>::Params::MODULUS_MINUS_ONE_DIV_TWO,
        )
        .unwrap();

        let square_root_pow = {
            let tmp: BigUint = <<P as BnParameters>::Fp as PrimeField>::Params::MODULUS
                .into()
                .add(1u64)
                .div(4u64);
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

            b,
            b_plus_one,

            minus_one,
            minus_3,
            minus_4_times_b_plus_1,

            inv_2,
            legendre_2,

            sqrt_minus_3,
            minus_sqrt_minus_3,
            sqrt_minus_3_minus_1_div_2,
            minus_sqrt_minus_3_div_2,

            q_minus_1_div_2,

            square_root_pow,
            phantom: PhantomData,
        }
    }

    pub fn encode<R: RngCore>(&self, val: P::Fp, rng: &mut R) -> (G1Affine<P>, DecodeHint) {
        // The following algorithm from [FT10] is equivalent to the original formulas for x_1, x_2, x_3
        // [FT10]: Pierre-Alain Fouque and Mehdi Tibouchi, "Indifferentiable Hashing to Barreto–Naehrig Curves", LATINCRYPT 2012.

        // val is v on the paper
        // w is 1/u; w.inverse() = u
        let w =
            self.minus_sqrt_minus_3 * &val * &(self.b_plus_one + &val.square()).inverse().unwrap();

        let x1 = self.sqrt_minus_3_minus_1_div_2 + val * &w;
        let x2 = self.minus_one - &x1;

        let u = w.inverse().unwrap_or_default();
        let x3 = <P as BnParameters>::Fp::one() + &u.square();

        // Sample r1 and r2 for data-independent-time masking
        let r1 = P::Fp::rand(rng);
        let r2 = P::Fp::rand(rng);

        // Compute the Legendre symbols of (r1 ^ 2 * (x1 ^ 3 + b)) and (r2 ^ 2 * (x2 ^ 3 + b))
        let alpha = self.compute_legendre_symbol(r1.square() * &(x1.square() * &x1 + &self.b));
        let beta = self.compute_legendre_symbol(r2.square() * &(x2.square() * &x2 + &self.b));

        let idx = (((alpha - 1) * beta + 3) % 3 + 1) as u8;

        let x = {
            let mut x = x1;

            let mut sel = idx.eq(&2u8) as u8;
            x = x * &P::Fp::from((1 - sel) as u128) + x2 * &P::Fp::from(sel as u128);

            sel = idx.eq(&3u8) as u8;
            x = x * &P::Fp::from((1 - sel) as u128) + x3 * &P::Fp::from(sel as u128);

            x
        };

        let mut y = self.compute_square_root(x * &x.square() + &self.b);

        // Compute the special character
        let sgn_cur = if y.cmp(&self.q_minus_1_div_2) == Ordering::Less {
            1
        } else {
            -1
        };

        let sgn_expected = self.compute_character(idx, val, u);

        if sgn_cur != sgn_expected {
            y = -y;
        }

        // Output the point * character;
        let point = G1Affine::<P>::new(x.clone(), y, false);
        assert!(point.is_on_curve());

        let decode_hint = {
            let mut x = 1;
            x.conditional_assign(&2u8, idx.ct_eq(&2u8));

            let sgn_u_cur = if u.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                1i8
            } else {
                -1i8
            };

            x.conditional_assign(&3u8, idx.ct_eq(&3u8).bitand(sgn_u_cur.ct_eq(&1i8)));
            x.conditional_assign(&4u8, idx.ct_eq(&3u8).bitand(sgn_u_cur.ct_eq(&-1i8)));

            x
        };

        (point, decode_hint)
    }

    pub fn compute_character(&self, idx: u8, val: P::Fp, u: P::Fp) -> i32 {
        if idx == 1 || idx == 2 {
            // CASE 1
            if val.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                1
            } else {
                -1
            }
        } else {
            // CASE 2
            let delta_sqrt =
                self.compute_square_root(self.minus_3 * &u.square() + &self.minus_4_times_b_plus_1);
            let comp_number = u * self.minus_sqrt_minus_3_div_2 - &(delta_sqrt * &self.inv_2);
            if val == comp_number {
                1
            } else {
                -1
            }
        }
    }

    #[inline]
    pub fn compute_legendre_symbol(&self, val: P::Fp) -> i32 {
        // Compute the Legendre symbol via the law of quadratic reciprocity (in the Jacobi case).
        assert!(!val.is_zero());

        let p = unsafe {
            let mut p = MaybeUninit::uninit();
            gmp::mpz_init_set_ui(p.as_mut_ptr(), 0u64);

            let mut p = p.assume_init();

            let repr = val.into_repr();
            let limbs: &[u64] = AsRef::<[u64]>::as_ref(&repr);
            for limb in limbs.iter().rev() {
                gmp::mpz_mul_2exp(&mut p, &p, 64);
                gmp::mpz_add_ui(&mut p, &p, *limb);
            }

            p
        };

        unsafe { gmp::mpz_jacobi(&p, &self.q) }
    }

    #[inline]
    pub fn compute_square_root(&self, val: P::Fp) -> P::Fp {
        val.pow(&self.square_root_pow)
    }

    fn helper_decode_attempt_1_2(&self, x: P::Fp, y: P::Fp) -> Option<P::Fp> {
        let step_1 = (x - &self.sqrt_minus_3_minus_1_div_2).neg().inverse();
        if step_1.is_some() {
            let step_2 = (step_1.unwrap() * &self.sqrt_minus_3 + &self.minus_one).inverse();
            if step_2.is_some() {
                let step_3 = step_2.unwrap() * &self.b_plus_one;
                if self.compute_legendre_symbol(step_3) == -1 {
                    None
                } else {
                    let step_4 = self.compute_square_root(step_3);

                    let sgn_cur = if step_4.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                        1
                    } else {
                        -1
                    };

                    let sgn_expected = if y.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                        1
                    } else {
                        -1
                    };

                    if sgn_cur != sgn_expected {
                        Some(step_4.neg())
                    } else {
                        Some(step_4)
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn helper_decode_attempt_3_4(&self, u: P::Fp, y: P::Fp) -> Option<P::Fp> {
        let step_1 = u.square() * &self.minus_3 + &self.minus_4_times_b_plus_1;

        if self.compute_legendre_symbol(step_1) == -1 {
            None
        } else {
            let mid = self.minus_sqrt_minus_3_div_2 * &u;
            let step_2 = self.compute_square_root(step_1) * &self.inv_2;

            let sgn_expected = if y.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                1
            } else {
                -1
            };

            if sgn_expected == 1 {
                Some(mid - &step_2)
            } else {
                Some(mid + &step_2)
            }
        }
    }

    pub fn decode_without_hints(&self, p: G1Affine<P>) -> [Option<P::Fp>; 4] {
        let mut res = [None; 4];

        // attempt 1: x = x1
        let attempt_1_t = self.helper_decode_attempt_1_2(p.x, p.y);
        res[0] = attempt_1_t;

        // attempt 2: x = x2
        let attempt_2_t = self.helper_decode_attempt_1_2((p.x + &P::Fp::one()).neg(), p.y);
        res[1] = attempt_2_t;

        // attempt 3: x = x3, u = sqrt{x3 - 1}
        // attempt 4: x = x4, u = -sqrt{x3 - 1}
        let mut attempt_3_t = None;
        let mut attempt_4_t = None;

        let x_minus_one = p.x - &P::Fp::one();
        if self.compute_legendre_symbol(x_minus_one) != -1 {
            let sqrt_x_minus_one = self.compute_square_root(x_minus_one);

            if sqrt_x_minus_one.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                attempt_3_t = self.helper_decode_attempt_3_4(sqrt_x_minus_one, p.y);
                attempt_4_t = self.helper_decode_attempt_3_4(sqrt_x_minus_one.neg(), p.y);
            } else {
                attempt_3_t = self.helper_decode_attempt_3_4(sqrt_x_minus_one.neg(), p.y);
                attempt_4_t = self.helper_decode_attempt_3_4(sqrt_x_minus_one, p.y);
            }
        }
        res[2] = attempt_3_t;
        res[3] = attempt_4_t;

        res
    }

    pub fn decode_with_hints(&self, p: G1Affine<P>, hint: u8) -> P::Fp {
        if hint == 1 {
            self.helper_decode_attempt_1_2(p.x, p.y).unwrap()
        } else if hint == 2 {
            self.helper_decode_attempt_1_2((p.x + &P::Fp::one()).neg(), p.y)
                .unwrap()
        } else {
            let x_minus_one = p.x - &P::Fp::one();
            let sqrt_x_minus_one = self.compute_square_root(x_minus_one);

            if hint == 3 {
                if sqrt_x_minus_one.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                    self.helper_decode_attempt_3_4(sqrt_x_minus_one, p.y)
                        .unwrap()
                } else {
                    self.helper_decode_attempt_3_4(sqrt_x_minus_one.neg(), p.y)
                        .unwrap()
                }
            } else {
                if sqrt_x_minus_one.cmp(&self.q_minus_1_div_2) == Ordering::Less {
                    self.helper_decode_attempt_3_4(sqrt_x_minus_one.neg(), p.y)
                        .unwrap()
                } else {
                    self.helper_decode_attempt_3_4(sqrt_x_minus_one, p.y)
                        .unwrap()
                }
            }
        }
    }
}

impl<P: BnParameters> Drop for Encoder<P> {
    fn drop(&mut self) {
        unsafe { gmp::mpz_clear(&mut self.q) }
    }
}

#[cfg(test)]
mod test {
    use crate::curve_bn446::Parameters as Bn446Parameters;
    use crate::message_encoding::Encoder;
    use ark_ec::bn::BnParameters;
    use ark_ff::{LegendreSymbol, SquareRootField, Zero};
    use ark_std::mem::MaybeUninit;
    use ark_std::ops::ShlAssign;
    use ark_std::str::FromStr;
    use ark_std::UniformRand;
    use gmp_mpfr_sys::gmp;
    use num_bigint::BigUint;

    const REPETITIONS: u64 = 100;

    #[test]
    fn test_precomputation() {
        let encoder = Encoder::<Bn446Parameters>::new();

        // q = 102211695604069718983520304652693874995639508460729604902280098199792736381528662976886082950231100101353700265360419596271313339023463
        unsafe {
            let mut expected_q = MaybeUninit::uninit();
            gmp::mpz_init_set_str(expected_q.as_mut_ptr(), "102211695604069718983520304652693874995639508460729604902280098199792736381528662976886082950231100101353700265360419596271313339023463\x00".as_ptr() as *const i8, 10);
            let mut expected_q = expected_q.assume_init();

            assert_eq!(gmp::mpz_cmp(&expected_q, &encoder.q), 0);

            gmp::mpz_clear(&mut expected_q);
        }

        // b
        assert_eq!(BigUint::from_str("257").unwrap(), encoder.b.into());

        // b_plus_one
        assert_eq!(BigUint::from_str("258").unwrap(), encoder.b_plus_one.into());

        // minus_one
        assert_eq!(BigUint::from_str("102211695604069718983520304652693874995639508460729604902280098199792736381528662976886082950231100101353700265360419596271313339023462").unwrap(), encoder.minus_one.into());

        // minus_3
        assert_eq!(BigUint::from_str("102211695604069718983520304652693874995639508460729604902280098199792736381528662976886082950231100101353700265360419596271313339023460").unwrap(), encoder.minus_3.into());

        // minus_4_times_b_plus_1
        assert_eq!(BigUint::from_str("102211695604069718983520304652693874995639508460729604902280098199792736381528662976886082950231100101353700265360419596271313339022431").unwrap(), encoder.minus_4_times_b_plus_1.into());

        // inv_2
        assert_eq!(BigUint::from_str("51105847802034859491760152326346937497819754230364802451140049099896368190764331488443041475115550050676850132680209798135656669511732").unwrap(), encoder.inv_2.into());

        // legendre_2
        assert_eq!(encoder.legendre_2, 1);

        // sqrt_minus_3
        assert_eq!(BigUint::from_str("78741026092188429277422585721872241941339850196332811688266123445560154836806169644207872181933178973").unwrap(), encoder.sqrt_minus_3.into());

        // minus_sqrt_minus_3
        assert_eq!(BigUint::from_str("102211695604069718983520304652693796254613416272300327479694376327550795041678466644074394684107654541198863459190775388399131405844490").unwrap(), encoder.minus_sqrt_minus_3.into());

        // sqrt_minus_3_minus_1_div_2
        assert_eq!(BigUint::from_str("39370513046094214638711292860936120970669925098166405844133061722780077418403084822103936090966589486").unwrap(), encoder.sqrt_minus_3_minus_1_div_2.into());

        // minus_sqrt_minus_3_div_2
        assert_eq!(BigUint::from_str("51105847802034859491760152326346898127306708136150163739847188163775397520839233322037197342053827270599431729595387694199565702922245").unwrap(), encoder.minus_sqrt_minus_3_div_2.into());

        // q_minus_1_div_2
        assert_eq!(BigUint::from_str("51105847802034859491760152326346937497819754230364802451140049099896368190764331488443041475115550050676850132680209798135656669511731").unwrap(), encoder.q_minus_1_div_2.into());

        // square_root_pow
        let mut square_root_pow_biguint = BigUint::zero();
        for limb in encoder.square_root_pow.iter().rev() {
            square_root_pow_biguint.shl_assign(64);
            square_root_pow_biguint += *limb;
        }
        assert_eq!(square_root_pow_biguint, BigUint::from_str("25552923901017429745880076163173468748909877115182401225570024549948184095382165744221520737557775025338425066340104899067828334755866").unwrap());
    }

    #[test]
    fn test_square_root() {
        let mut rng = ark_std::test_rng();
        let encoder = Encoder::<Bn446Parameters>::new();

        for _ in 0..REPETITIONS {
            let mut a = <Bn446Parameters as BnParameters>::Fp::rand(&mut rng);
            a = a * &a;

            let sqrt_a = encoder.compute_square_root(a);
            assert_eq!(sqrt_a * &sqrt_a, a);
        }
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
                LegendreSymbol::QuadraticNonResidue => -1,
            };

            assert_eq!(res_encoder, res_standard);
        }
    }

    #[test]
    fn test_encoding() {
        let mut rng = ark_std::test_rng();
        let encoder = Encoder::<Bn446Parameters>::new();

        let (p1, h1) = encoder.encode(<Bn446Parameters as BnParameters>::Fp::from(3u64), &mut rng);
        assert_eq!(BigUint::from_str("36716321155346290056326261881547168995119143630874288596213978910008386805926472362186816803935134015").unwrap(), p1.x.into());
        assert_eq!(BigUint::from_str("48674267463598597561434421411169385221111256267026887203971795345888926403911609875517263053966874364761655980041561930937695329940019").unwrap(), p1.y.into());
        assert_eq!(h1, 1);

        let (p2, h2) = encoder.encode(<Bn446Parameters as BnParameters>::Fp::from(4u64), &mut rng);
        assert_eq!(BigUint::from_str("31334972374970278812466078798636043693319567722267679721381955847596039977834065451781812728534582101222538572057475536256409877968303").unwrap(), p2.x.into());
        assert_eq!(BigUint::from_str("26562368505820453331331139235045952220077135576631966471336165996466766241424839991915013754701621604012397377681212063456304981999664").unwrap(), p2.y.into());
        assert_eq!(h2, 2);

        let (p3, h3) = encoder.encode(<Bn446Parameters as BnParameters>::Fp::from(1u64), &mut rng);
        assert_eq!(BigUint::from_str("34070565201356572994506768217564624998546502820243201634093366066597578793842887658962027650077033367117900088453473198757104446318795").unwrap(), p3.x.into());
        assert_eq!(BigUint::from_str("73860803538922718841691690520324796530042249436798270933990858732782956505671392697658857721094746164733733326003807537837075468843619").unwrap(), p3.y.into());
        assert!(h3 == 3 || h3 == 4);
    }

    #[test]
    fn test_decoding_simple() {
        let mut rng = ark_std::test_rng();
        let encoder = Encoder::<Bn446Parameters>::new();

        let (p1, _) = encoder.encode(<Bn446Parameters as BnParameters>::Fp::from(3u64), &mut rng);
        let (p2, _) = encoder.encode(<Bn446Parameters as BnParameters>::Fp::from(4u64), &mut rng);
        let (p3, _) = encoder.encode(<Bn446Parameters as BnParameters>::Fp::from(1u64), &mut rng);

        let decode_p1 = encoder.decode_without_hints(p1);
        let decode_p2 = encoder.decode_without_hints(p2);
        let decode_p3 = encoder.decode_without_hints(p3);

        assert_eq!(
            decode_p1[0],
            Some(<Bn446Parameters as BnParameters>::Fp::from(3u64))
        );
        assert_eq!(
            decode_p2[1],
            Some(<Bn446Parameters as BnParameters>::Fp::from(4u64))
        );
        assert_eq!(
            decode_p3[2],
            Some(<Bn446Parameters as BnParameters>::Fp::from(1u64))
        );
    }

    #[test]
    fn test_decoding_rand() {
        let mut rng = ark_std::test_rng();

        for _ in 0..REPETITIONS {
            let t = <Bn446Parameters as BnParameters>::Fp::rand(&mut rng);

            let encoder = Encoder::<Bn446Parameters>::new();
            let (p, h) = encoder.encode(t, &mut rng);

            let t_recovered = encoder.decode_with_hints(p, h);
            assert_eq!(t, t_recovered);
        }
    }
}
