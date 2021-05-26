use ark_ec::ProjectiveCurve;
use ark_ff::PrimeField;
use ark_std::{marker::PhantomData, vec::Vec, UniformRand};

#[derive(Clone)]
pub struct ElGamalPublicParameters<G: ProjectiveCurve> {
    pub g: G,
}

#[derive(Clone)]
pub struct ElGamalSecretKey<G: ProjectiveCurve> {
    pub scalar_x: Vec<G::ScalarField>,
}

#[derive(Clone)]
pub struct ElGamalPublicKey<G: ProjectiveCurve> {
    pub pp: ElGamalPublicParameters<G>,
    pub y: Vec<G>,
}

#[derive(Clone)]
pub struct ElGamalCiphertext<G: ProjectiveCurve> {
    pub r: G,
    pub e: Vec<G>,
}

pub struct ElGamalEncryption<G: ProjectiveCurve> {
    pub pairing_engine_phantom: PhantomData<G>,
}

impl<G: ProjectiveCurve> ElGamalEncryption<G> {
    pub fn setup<R: ark_std::rand::Rng>(rng: &mut R) -> ElGamalPublicParameters<G> {
        let g: G = G::rand(rng);

        ElGamalPublicParameters::<G> { g }
    }

    pub fn key_generation<R: ark_std::rand::Rng>(
        pp: &ElGamalPublicParameters<G>,
        len: usize,
        rng: &mut R,
    ) -> (ElGamalSecretKey<G>, ElGamalPublicKey<G>) {
        let mut scalar_x = Vec::<G::ScalarField>::new();

        for _ in 0..len {
            scalar_x.push(G::ScalarField::rand(rng));
        }

        let mut y = Vec::<G>::new();

        for i in 0..len {
            y.push(pp.g.mul(&scalar_x[i].into_repr()));
        }

        let sk = ElGamalSecretKey::<G> { scalar_x };

        let pk = ElGamalPublicKey::<G> {
            pp: (*pp).clone(),
            y,
        };

        (sk, pk)
    }

    pub fn encrypt<R: ark_std::rand::Rng>(
        pk: &ElGamalPublicKey<G>,
        plaintext: &Vec<G>,
        rng: &mut R,
    ) -> ElGamalCiphertext<G> {
        assert!(plaintext.len() <= pk.y.len());
        let len = plaintext.len();

        let scalar_r = G::ScalarField::rand(rng);
        let r = pk.pp.g.mul(&scalar_r.into_repr());

        let mut e = Vec::<G>::new();

        for i in 0..len {
            e.push(plaintext[i] + pk.y[i].mul(&scalar_r.into_repr()));
        }
        ElGamalCiphertext::<G> { r, e }
    }

    pub fn decrypt(sk: &ElGamalSecretKey<G>, ciphertext: &ElGamalCiphertext<G>) -> Vec<G> {
        let mut plaintext = Vec::new();
        let len = sk.scalar_x.len();

        for i in 0..len {
            plaintext.push(ciphertext.e[i] - ciphertext.r.mul(&sk.scalar_x[i].into_repr()));
        }

        plaintext
    }

    pub fn rerand<R: ark_std::rand::Rng>(
        pk: &ElGamalPublicKey<G>,
        ciphertext: &ElGamalCiphertext<G>,
        rng: &mut R,
    ) -> ElGamalCiphertext<G> {
        let len = ciphertext.e.len();
        let scalar_r_new = G::ScalarField::rand(rng);

        let r_new = ciphertext.r + pk.pp.g.mul(&scalar_r_new.into_repr());
        let mut e_new = Vec::<G>::new();
        for i in 0..len {
            e_new.push(ciphertext.e[i] + pk.y[i].mul(&scalar_r_new.into_repr()));
        }

        ElGamalCiphertext::<G> { r: r_new, e: e_new }
    }
}

#[cfg(test)]
mod test {
    use crate::elgamal_encryption::ElGamalEncryption;
    use ark_bls12_381::G1Projective;
    use ark_std::UniformRand;

    #[test]
    fn test_encrypt_decrypt() {
        let mut rng = ark_std::test_rng();
        let len = 10;

        let mut pt = Vec::new();
        for _ in 0..len {
            pt.push(G1Projective::rand(&mut rng));
        }

        let pp = ElGamalEncryption::<G1Projective>::setup(&mut rng);
        let (sk, pk) = ElGamalEncryption::<G1Projective>::key_generation(&pp, len, &mut rng);

        let ct = ElGamalEncryption::encrypt(&pk, &pt, &mut rng);
        let pt_recovered = ElGamalEncryption::decrypt(&sk, &ct);

        for i in 0..len {
            assert!(
                pt[i].eq(&pt_recovered[i]),
                "Decrypted results do not match the plaintexts."
            );
        }
    }

    #[test]
    fn test_rerandomization() {
        let mut rng = ark_std::test_rng();
        let len = 10;

        let mut pt = Vec::new();
        for _ in 0..len {
            pt.push(G1Projective::rand(&mut rng));
        }

        let pp = ElGamalEncryption::<G1Projective>::setup(&mut rng);
        let (sk, pk) = ElGamalEncryption::<G1Projective>::key_generation(&pp, len, &mut rng);

        let ct = ElGamalEncryption::encrypt(&pk, &pt, &mut rng);
        let ct_rerand = ElGamalEncryption::rerand(&pk, &ct, &mut rng);
        let pt_recovered = ElGamalEncryption::decrypt(&sk, &ct_rerand);

        for i in 0..len {
            assert!(
                pt[i].eq(&pt_recovered[i]),
                "Decrypted results of rerandomized ciphertexts do not match the plaintexts."
            );
        }
    }
}
