use ark_ec::ProjectiveCurve;
use ark_std::marker::PhantomData;
use ark_std::vec::Vec;
use ark_std::UniformRand;

#[derive(Clone)]
pub struct ShachamPublicParameters<G: ProjectiveCurve> {
    pub u: G,
    pub v: G,
    pub w: G,
}

#[derive(Clone)]
pub struct ShachamSecretKey<G: ProjectiveCurve> {
    pub scalar_x: Vec<G::ScalarField>,
    pub scalar_y: Vec<G::ScalarField>,
    pub scalar_z: Vec<G::ScalarField>,
}

#[derive(Clone)]
pub struct ShachamPublicKey<G: ProjectiveCurve> {
    pub pp: ShachamPublicParameters<G>,
    pub y: Vec<G>,
    pub z: Vec<G>,
}

#[derive(Clone)]
pub struct ShachamCiphertext<G: ProjectiveCurve> {
    pub r1: G,
    pub r2: G,
    pub r3: G,
    pub e: Vec<G>,
}

pub struct ShachamEncryption<G: ProjectiveCurve> {
    pub pairing_engine_phantom: PhantomData<G>,
}

impl<G: ProjectiveCurve> ShachamEncryption<G> {
    pub fn setup<R: ark_std::rand::Rng>(rng: &mut R) -> ShachamPublicParameters<G> {
        let u: G = G::rand(rng);
        let v: G = G::rand(rng);
        let w: G = G::rand(rng);

        ShachamPublicParameters::<G> { u, v, w }
    }

    pub fn key_generation<R: ark_std::rand::Rng>(
        pp: &ShachamPublicParameters<G>,
        len: usize,
        rng: &mut R,
    ) -> (ShachamSecretKey<G>, ShachamPublicKey<G>) {
        let mut scalar_x = Vec::<G::ScalarField>::new();
        let mut scalar_y = Vec::<G::ScalarField>::new();
        let mut scalar_z = Vec::<G::ScalarField>::new();

        for _ in 0..len {
            scalar_x.push(G::ScalarField::rand(rng));
            scalar_y.push(G::ScalarField::rand(rng));
            scalar_z.push(G::ScalarField::rand(rng));
        }

        let mut y = Vec::<G>::new();
        let mut z = Vec::<G>::new();

        for i in 0..len {
            y.push(pp.u.mul(scalar_x[i].into()) + pp.w.mul(scalar_z[i].into()));
            z.push(pp.v.mul(scalar_y[i].into()) + pp.w.mul(scalar_z[i].into()));
        }

        let sk = ShachamSecretKey::<G> {
            scalar_x,
            scalar_y,
            scalar_z,
        };

        let pk = ShachamPublicKey::<G> {
            pp: (*pp).clone(),
            y,
            z,
        };

        (sk, pk)
    }

    pub fn encrypt<R: ark_std::rand::Rng>(
        pk: &ShachamPublicKey<G>,
        plaintext: &Vec<G>,
        rng: &mut R,
    ) -> ShachamCiphertext<G> {
        assert!(plaintext.len() <= pk.y.len());
        let len = plaintext.len();

        let a = G::ScalarField::rand(rng);
        let b = G::ScalarField::rand(rng);

        let r1 = pk.pp.u.mul(a.into());
        let r2 = pk.pp.v.mul(b.into());
        let r3 = pk.pp.w.mul((a + b).into());

        let mut e = Vec::<G>::new();

        for i in 0..len {
            e.push(plaintext[i] + pk.y[i].mul(a.into()) + pk.z[i].mul(b.into()));
        }
        ShachamCiphertext::<G> { r1, r2, r3, e }
    }

    pub fn decrypt(sk: &ShachamSecretKey<G>, ciphertext: &ShachamCiphertext<G>) -> Vec<G> {
        let mut plaintext = Vec::new();

        let len = sk.scalar_x.len();

        for i in 0..len {
            plaintext.push(
                ciphertext.e[i]
                    - ciphertext.r1.mul(sk.scalar_x[i].into())
                    - ciphertext.r2.mul(sk.scalar_y[i].into())
                    - ciphertext.r3.mul(sk.scalar_z[i].into()),
            );
        }

        plaintext
    }

    pub fn rerand<R: ark_std::rand::Rng>(
        pk: &ShachamPublicKey<G>,
        ciphertext: &ShachamCiphertext<G>,
        rng: &mut R,
    ) -> ShachamCiphertext<G> {
        let len = ciphertext.e.len();

        let a_new = G::ScalarField::rand(rng);
        let b_new = G::ScalarField::rand(rng);

        let r1_new = ciphertext.r1 + pk.pp.u.mul(a_new.into());
        let r2_new = ciphertext.r2 + pk.pp.v.mul(b_new.into());
        let r3_new = ciphertext.r3 + pk.pp.w.mul((a_new + b_new).into());

        let mut e_new = Vec::<G>::new();

        for i in 0..len {
            e_new.push(ciphertext.e[i] + pk.y[i].mul(a_new.into()) + pk.z[i].mul(b_new.into()));
        }

        ShachamCiphertext::<G> {
            r1: r1_new,
            r2: r2_new,
            r3: r3_new,
            e: e_new,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::shacham_encryption::ShachamEncryption;
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

        let pp = ShachamEncryption::<G1Projective>::setup(&mut rng);
        let (sk, pk) = ShachamEncryption::<G1Projective>::key_generation(&pp, len, &mut rng);

        let ct = ShachamEncryption::encrypt(&pk, &pt, &mut rng);
        let pt_recovered = ShachamEncryption::decrypt(&sk, &ct);

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

        let pp = ShachamEncryption::<G1Projective>::setup(&mut rng);
        let (sk, pk) = ShachamEncryption::<G1Projective>::key_generation(&pp, len, &mut rng);

        let ct = ShachamEncryption::encrypt(&pk, &pt, &mut rng);
        let ct_rerand = ShachamEncryption::rerand(&pk, &ct, &mut rng);
        let pt_recovered = ShachamEncryption::decrypt(&sk, &ct_rerand);

        for i in 0..len {
            assert!(
                pt[i].eq(&pt_recovered[i]),
                "Decrypted results of rerandomized ciphertexts do not match the plaintexts."
            );
        }
    }
}
