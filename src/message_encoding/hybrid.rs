use crate::message_encoding::{DecodeHint, Encoder};
use ark_ec::bn::{BnParameters, G1Affine};
use ark_ff::{BigInteger, PrimeField};
use ark_std::rand::RngCore;
use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2s;

// The length of the hash value in the hybrid embedding,
// 80 is chosen heuristically treating the VarBlake2 as a random oracle.
//
// Note that we are not using the collision resistance of the hash function,
// we are using the pseudorandomness of the hash function, treated as the random oracle.
const HASH_LEN: usize = 80;

#[derive(Default)]
pub struct HybridEncoder<P: BnParameters> {
    // The number of bytes for an embed-direct point
    pub num_bytes_per_point: usize,
    // The number of data points in a group
    pub num_data_points: usize,
    pub encoder: Encoder<P>,
}

impl<P: BnParameters> HybridEncoder<P> {
    fn new() -> Self {
        let capacity = P::Fp::size_in_bits() - 1;
        let num_bytes_per_point = capacity >> 3; // divide the capacity directly by 8

        assert!(num_bytes_per_point > 10);
        let num_points = (num_bytes_per_point * 8 - HASH_LEN) / 2;

        Self {
            num_bytes_per_point: num_bytes_per_point,
            num_data_points: num_points,
            encoder: Encoder::<P>::new(),
        }
    }

    pub fn get_capacity(&self) -> usize {
        // return how many bytes the hybrid encoder will encode in one pass
        return self.num_bytes_per_point * self.num_data_points;
    }

    pub fn encode<R: RngCore>(&self, bytes: &[u8], rng: &mut R) -> Vec<G1Affine<P>> {
        assert_eq!(bytes.len(), self.get_capacity());

        // split and encode them to self.num_points points
        // store the hints
        let mut points = Vec::<G1Affine<P>>::new();
        let mut hints = Vec::<DecodeHint>::new();

        for i in 0..self.num_data_points {
            // convert the bytes to a field element using P::Fp::from_le_bytes_mod_order
            let field_element = P::Fp::from_le_bytes_mod_order(
                &bytes[i * self.num_bytes_per_point..(i + 1) * self.num_bytes_per_point],
            );
            let (point, hint) = self.encoder.encode(field_element, rng);
            points.push(point);
            hints.push(hint);
        }

        let mut hints_bool: Vec<u8> = Vec::new();
        let mut hints_u8: Vec<u8> = Vec::new();
        for hint in hints {
            match hint {
                1 => {
                    hints_bool.push(0);
                    hints_bool.push(0);
                }
                2 => {
                    hints_bool.push(1);
                    hints_bool.push(0);
                }
                3 => {
                    hints_bool.push(0);
                    hints_bool.push(1);
                }
                4 => {
                    hints_bool.push(1);
                    hints_bool.push(1);
                }
                _ => panic!("Illegal hint."),
            }
        }

        hints_bool.chunks_exact(8).for_each(|bits| {
            let mut num: u8 = 0;
            for bit in bits.iter().rev() {
                num *= 2;
                num += bit;
            }
            hints_u8.push(num);
        });

        // compute H(hints) and take the first 80-bit, which would be 10 bytes
        let mut hasher = VarBlake2s::new(10).unwrap();
        hasher.update(hints_u8.clone());

        let mut res = Vec::new();
        hasher.finalize_variable_reset(|r| res = r.to_vec());

        hints_u8.extend(res.clone());

        // encode the hints || H(hints)
        let field_element = P::Fp::from_le_bytes_mod_order(&hints_u8);
        let (point, _) = self.encoder.encode(field_element, rng);

        points.push(point);
        points
    }

    pub fn decode(&self, points: &[G1Affine<P>]) -> Vec<u8> {
        // take the last point out
        let last_point = points.last().copied().unwrap();

        // decode the last point
        let decoded_last_point = self.encoder.decode_without_hints(last_point);

        // check the candidate numbers, see which one matches the pattern, this involves a check of the H(.)
        let mut hints_raw = Vec::new();

        for candidate in decoded_last_point.iter() {
            if let Some(field_element) = candidate {
                let hints = field_element.into_repr().to_bytes_le();
                let index = self.num_bytes_per_point - (HASH_LEN / 8);
                let hashed = hints.get(index..index + (HASH_LEN / 8)).unwrap();
                let values = hints.get(..index).unwrap();
                let mut hasher = VarBlake2s::new(10).unwrap();
                hasher.update(values);

                let mut res = Vec::new();
                hasher.finalize_variable_reset(|r| res = r.to_vec());

                if res == hashed {
                    hints_raw = values.to_vec();
                }
            }
        }
        assert!(!hints_raw.is_empty());

        let mut hints: Vec<u8> = Vec::new();
        for num in hints_raw.iter() {
            let n1: u8 = (num & 0b00000011) + 1;
            let n2: u8 = ((num & 0b00001100) >> 2) + 1;
            let n3: u8 = ((num & 0b00110000) >> 4) + 1;
            let n4: u8 = ((num & 0b11000000) >> 6) + 1;
            hints.push(n1);
            hints.push(n2);
            hints.push(n3);
            hints.push(n4);
        }

        let mut ret: Vec<u8> = Vec::new();

        for (i, point) in points.iter().take(self.num_data_points).enumerate() {
            ret.extend(
                &self
                    .encoder
                    .decode_with_hints(*point, hints[i])
                    .into_repr()
                    .to_bytes_le()[..self.num_bytes_per_point],
            )
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use crate::curve_bn446::Parameters as Bn446Parameters;
    use crate::message_encoding::hybrid::HybridEncoder;
    use ark_std::rand::RngCore;

    const REPETITIONS: u64 = 10;

    #[test]
    fn test_decoding_rand() {
        let mut rng = ark_std::test_rng();

        for _ in 0..REPETITIONS {
            let encoder = HybridEncoder::<Bn446Parameters>::new();
            let num_bytes = encoder.get_capacity();

            let mut test_bytes = vec![0u8; num_bytes];
            rng.fill_bytes(&mut test_bytes[..]);

            let points = encoder.encode(&test_bytes, &mut rng);
            let test_bytes_recovered = encoder.decode(&points);

            assert_eq!(test_bytes, test_bytes_recovered);
        }
    }
}
