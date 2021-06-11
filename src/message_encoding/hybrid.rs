use crate::message_encoding::{DecodeHint, Encoder};
use ark_ec::bn::{BnParameters, G1Affine};
use ark_ff::PrimeField;
use ark_std::rand::RngCore;
use blake2::digest::{VariableOutput, Update};
use blake2::VarBlake2s;
use std::sync::atomic::AtomicBool;

#[derive(Default)]
pub struct HybridEncoder<P: BnParameters> {
    // The number of bytes for an embed-direct point
    pub num_bytes_direct: usize,
    // The number of points in a group
    pub num_points: usize,
    pub encoder: Encoder<P>,
}

impl<P: BnParameters> HybridEncoder<P> {
    fn new() -> Self {
        let capacity = P::Fp::size_in_bits() - 1;
        let num_bytes_direct = capacity >> 3; // divide the capacity directly by 8

        assert!(num_bytes_direct > 10);
        let num_points = (num_bytes_direct * 8 - 80) / 2;

        Self {
            num_bytes_direct,
            num_points,
            encoder: Encoder::<P>::new(),
        }
    }

    pub fn get_capacity(&self) -> usize {
        // return how many bytes the hybrid encoder will encode in one pass
        return self.num_bytes_direct * self.num_points;
    }

    pub fn encode<R: RngCore>(&self, bytes: &[u8], rng: &mut R) -> Vec<G1Affine<P>> {
        assert_eq!(bytes.len(), self.get_capacity());

        // split and encode them to self.num_points points
        // store the hints
        let mut points = Vec::<G1Affine<P>>::new();
        let mut hints = Vec::<DecodeHint>::new();

        for i in 0..self.num_points {
            // convert the bytes to a field element using P::Fp::from_le_bytes_mod_order
            let field_element = P::Fp::from_le_bytes_mod_order(&bytes[i * 8..(i + 1) * 8]);
            let (point, hint) = self.encoder.encode(field_element, rng);
            points.push(point);
            hints.push(hint);
        }

        let mut hints_bool: Vec<bool> = Vec::new();
        let mut hints_u8: Vec<u8> = Vec::new();
        for hint in hints {
            match hint {
                1 => {
                    hints_bool.push(false);
                    hints_bool.push(false);
                },
                2 => {
                    hints_bool.push(false);
                    hints_bool.push(true);
                },
                3 => {
                    hints_bool.push(true);
                    hints_bool.push(false);
                },
                4 => {
                    hints_bool.push(true);
                    hints_bool.push(true);
                },
                _ => panic!("Received hint that is not 1, 2, 3, or 4"),
            }
        }
        let mut cur_index: usize = 8;
        while cur_index <= hints_bool.len() {
            let mut num: u8 = 0;
            for i in (1..9).rev() {
                num = num << 1;
                match hints_bool[cur_index - i] {
                    true => num += 1,
                    false => (),
                }
            }
            hints_u8.push(num);
            cur_index += 8;
        }



        // convert the hint into 2 boolean values (true/false, 0/1) => Vec<bool>
        // write some code to convert Vec<bool> to Vec<u8>
        // compute H(hints) and take the first 80-bit, which would be 10 bytes
        // => make them into a field element
        let mut hasher = VarBlake2s::new(10).unwrap();
        hasher.update(hints_u8.clone());
        // use hasher.update to put data into the hasher
        let mut res = Vec::new();
        hasher.finalize_variable_reset(|r| res = r.to_vec());
        // use hasher.finalize_variable_reset to obtain the hash value and reset the hasher
        hints_u8.extend(res.clone());
        // encode the hints || H(hints)
        let field_element = P::Fp::from_le_bytes_mod_order(&hints_u8);
        let (point, hint) = self.encoder.encode(field_element, rng);

        // output
        points.push(point);
        points
        // unimplemented!()
    }

    pub fn decode(&self, points: &[G1Affine<P>]) -> Vec<u8> {
        // take the last point out

        // decode the last point

        // check the candidate numbers, see which one matches the pattern, this involves a check of the H(.)

        // get all the hints

        // there will be some conversions from the packed 2-bit hint information the actual hints (1-4)

        // decode the first N-1 points
        // self.encoder.decode_with_hints(?, ?) =>

        // arrange all the bytes together

        unimplemented!()
    }
}
