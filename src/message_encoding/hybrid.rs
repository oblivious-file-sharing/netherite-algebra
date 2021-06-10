use crate::message_encoding::{DecodeHint, Encoder};
use ark_ec::bn::{BnParameters, G1Affine};
use ark_ff::PrimeField;
use ark_std::rand::RngCore;
use blake2::digest::VariableOutput;
use blake2::VarBlake2s;

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

        // convert the hint into 2 boolean values (true/false, 0/1) => Vec<bool>
        // write some code to convert Vec<bool> to Vec<u8>
        // compute H(hints) and take the first 80-bit, which would be 10 bytes
        // => make them into a field element
        let mut hasher = VarBlake2s::new(10).unwrap();
        // use hasher.update to put data into the hasher
        // use hasher.finalize_variable_reset to obtain the hash value and reset the hasher

        // encode the hints || H(hints)

        // output

        unimplemented!()
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
