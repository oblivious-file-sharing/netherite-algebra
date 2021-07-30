use ark_ec::bn::{Bn, BnParameters, G1Affine, G1Prepared, G2Prepared, G2Projective};
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::Fp12;
use ark_std::marker::PhantomData;
use ark_std::rand::RngCore;
use ark_std::UniformRand;

pub trait GroupHasher<P: BnParameters, const L: usize> {
    type PubParam: Clone;
    type Hash: Clone + Eq;

    fn setup<R: RngCore>(rng: &mut R) -> Self::PubParam;
    fn eval(pp: &Self::PubParam, m: &[G1Affine<P>]) -> Self::Hash;
    fn check(pp: &Self::PubParam, m: &[G1Affine<P>], h: &Self::Hash) -> bool;
}

pub struct GroupHasherXDH<P: BnParameters, const L: usize> {
    phantom: PhantomData<P>,
}

impl<P: BnParameters, const L: usize> GroupHasher<P, L> for GroupHasherXDH<P, L> {
    type PubParam = Vec<G2Prepared<P>>;
    type Hash = Fp12<P::Fp12Params>;

    fn setup<R: RngCore>(rng: &mut R) -> Self::PubParam {
        let mut pp = Vec::new();
        for _ in 0..L {
            pp.push(G2Prepared::<P>::from(
                G2Projective::<P>::rand(rng).into_affine(),
            ))
        }
        pp
    }

    fn eval(pp: &Self::PubParam, m: &[G1Affine<P>]) -> Self::Hash {
        assert_eq!(m.len(), L);

        let mut miller_loop_list = Vec::<(G1Prepared<P>, G2Prepared<P>)>::new();
        for (a, b) in m
            .iter()
            .map(|x| G1Prepared::<P>::from((*x).clone()))
            .zip(pp.iter())
        {
            miller_loop_list.push((a, (*b).clone()));
        }
        let miller_loop_result = Bn::<P>::miller_loop(&miller_loop_list);
        Bn::<P>::final_exponentiation(&miller_loop_result).unwrap()
    }

    fn check(pp: &Self::PubParam, m: &[G1Affine<P>], h: &Self::Hash) -> bool {
        Self::eval(pp, m) == *h
    }
}

pub struct GroupHasherDLIN<P: BnParameters, const L: usize> {
    phantom: PhantomData<P>,
}

impl<P: BnParameters, const L: usize> GroupHasher<P, L> for GroupHasherDLIN<P, L> {
    type PubParam = (Vec<G2Prepared<P>>, Vec<G2Prepared<P>>);
    type Hash = (Fp12<P::Fp12Params>, Fp12<P::Fp12Params>);

    fn setup<R: RngCore>(rng: &mut R) -> Self::PubParam {
        let mut pp0 = Vec::new();
        for _ in 0..L {
            pp0.push(G2Prepared::<P>::from(
                G2Projective::<P>::rand(rng).into_affine(),
            ))
        }
        let mut pp1 = Vec::new();
        for _ in 0..L {
            pp1.push(G2Prepared::<P>::from(
                G2Projective::<P>::rand(rng).into_affine(),
            ))
        }
        (pp0, pp1)
    }

    fn eval(pp: &Self::PubParam, m: &[G1Affine<P>]) -> Self::Hash {
        assert_eq!(m.len(), L);

        let mut g_miller_loop_list = Vec::<(G1Prepared<P>, G2Prepared<P>)>::new();
        for (a, b) in m
            .iter()
            .map(|x| G1Prepared::<P>::from((*x).clone()))
            .zip(pp.0.iter())
        {
            g_miller_loop_list.push((a, (*b).clone()));
        }
        let g_miller_loop_result = Bn::<P>::miller_loop(&g_miller_loop_list);

        let mut h_miller_loop_list = Vec::<(G1Prepared<P>, G2Prepared<P>)>::new();
        for (a, b) in m
            .iter()
            .map(|x| G1Prepared::<P>::from((*x).clone()))
            .zip(pp.1.iter())
        {
            h_miller_loop_list.push((a, (*b).clone()));
        }
        let h_miller_loop_result = Bn::<P>::miller_loop(&h_miller_loop_list);
        (
            Bn::<P>::final_exponentiation(&g_miller_loop_result).unwrap(),
            Bn::<P>::final_exponentiation(&h_miller_loop_result).unwrap(),
        )
    }

    fn check(pp: &Self::PubParam, m: &[G1Affine<P>], h: &Self::Hash) -> bool {
        Self::eval(pp, m) == *h
    }
}
