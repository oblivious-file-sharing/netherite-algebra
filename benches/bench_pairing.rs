use ark_ec::{AffineCurve, PairingEngine};
use netherite_algebra::{curve_bn254::Bn254, curve_bn446::Bn446};

const NUM_REPETITIONS: u128 = 1000;

fn bench_miller_loop<PE: PairingEngine>() {
    let mut miller_loop_list = Vec::<(PE::G1Prepared, PE::G2Prepared)>::new();
    miller_loop_list.push((
        PE::G1Prepared::from(PE::G1Affine::prime_subgroup_generator()),
        PE::G2Prepared::from(PE::G2Affine::prime_subgroup_generator()),
    ));

    let start = ark_std::time::Instant::now();

    for _ in 0..NUM_REPETITIONS {
        let _ = PE::miller_loop(miller_loop_list.iter());
    }

    println!(
        "per-Miller loop: {} us",
        start.elapsed().as_micros() / NUM_REPETITIONS
    );
}

fn bench_final_exponentiation<PE: PairingEngine>() {
    let mut miller_loop_list = Vec::<(PE::G1Prepared, PE::G2Prepared)>::new();
    miller_loop_list.push((
        PE::G1Prepared::from(PE::G1Affine::prime_subgroup_generator()),
        PE::G2Prepared::from(PE::G2Affine::prime_subgroup_generator()),
    ));

    let res = PE::miller_loop(miller_loop_list.iter());

    let start = ark_std::time::Instant::now();

    for _ in 0..NUM_REPETITIONS {
        let _ = PE::final_exponentiation(&res);
    }

    println!(
        "per-final exponentiation: {} us",
        start.elapsed().as_micros() / NUM_REPETITIONS
    );
}

fn main() {
    println!("BN254");
    bench_miller_loop::<Bn254>();
    bench_final_exponentiation::<Bn254>();

    println!("BN446");
    bench_miller_loop::<Bn446>();
    bench_final_exponentiation::<Bn446>();
}
