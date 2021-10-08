// For benchmark, run:
//     RUSTFLAGS="-C target-feature=+bmi2,+adx" RAYON_NUM_THREADS=N cargo +nightly bench_encoding --no-default-features --features "std parallel asm" -- --nocapture
// where N is the number of threads you want to use (N = 1 for single-thread).

use ark_std::rand::RngCore;
use netherite_algebra::curve_bn254::Parameters as Bn254Parameters;
use netherite_algebra::curve_bn446::Parameters as Bn446Parameters;
use netherite_algebra::message_encoding::hybrid::HybridEncoder;

const NUM_REPETITIONS: u128 = 10;

unsafe fn bench_hybrid_encoding_bn254() {
    let mut rng = ark_std::test_rng();
    let encoder = HybridEncoder::<Bn254Parameters>::new();
    let num_bytes = encoder.get_capacity();

    let mut test_bytes = vec![0u8; num_bytes];
    rng.fill_bytes(&mut test_bytes[..]);

    let start = ark_std::time::Instant::now();

    for _ in 0..NUM_REPETITIONS {
        let _ = encoder.encode(&test_bytes, &mut rng);
    }

    println!(
        "per-byte encoding time for BN254: {} ns/byte",
        start.elapsed().as_nanos() / NUM_REPETITIONS / encoder.get_capacity() as u128
    );
}

unsafe fn bench_hybrid_decoding_bn254() {
    let mut rng = ark_std::test_rng();
    let encoder = HybridEncoder::<Bn254Parameters>::new();
    let num_bytes = encoder.get_capacity();

    let mut test_bytes = vec![0u8; num_bytes];
    rng.fill_bytes(&mut test_bytes[..]);

    let points = encoder.encode(&test_bytes, &mut rng);

    let start = ark_std::time::Instant::now();

    for _ in 0..NUM_REPETITIONS {
        let _ = encoder.decode(&points);
    }

    println!(
        "per-byte decoding time for BN254: {} ns/byte",
        start.elapsed().as_nanos() / NUM_REPETITIONS / encoder.get_capacity() as u128
    );
}

unsafe fn bench_hybrid_encoding_bn446() {
    let mut rng = ark_std::test_rng();
    let encoder = HybridEncoder::<Bn446Parameters>::new();
    let num_bytes = encoder.get_capacity();

    let mut test_bytes = vec![0u8; num_bytes];
    rng.fill_bytes(&mut test_bytes[..]);

    let start = ark_std::time::Instant::now();

    for _ in 0..NUM_REPETITIONS {
        let _ = encoder.encode(&test_bytes, &mut rng);
    }

    println!(
        "per-byte encoding time for BN446: {} ns/byte",
        start.elapsed().as_nanos() / NUM_REPETITIONS / encoder.get_capacity() as u128
    );
}

unsafe fn bench_hybrid_decoding_bn446() {
    let mut rng = ark_std::test_rng();
    let encoder = HybridEncoder::<Bn446Parameters>::new();
    let num_bytes = encoder.get_capacity();

    let mut test_bytes = vec![0u8; num_bytes];
    rng.fill_bytes(&mut test_bytes[..]);

    let points = encoder.encode(&test_bytes, &mut rng);

    let start = ark_std::time::Instant::now();

    for _ in 0..NUM_REPETITIONS {
        let _ = encoder.decode(&points);
    }

    println!(
        "per-byte decoding time for BN446: {} ns/byte",
        start.elapsed().as_nanos() / NUM_REPETITIONS / encoder.get_capacity() as u128
    );
}

fn main() {
    unsafe {
        bench_hybrid_encoding_bn254();
        bench_hybrid_decoding_bn254();

        bench_hybrid_encoding_bn446();
        bench_hybrid_decoding_bn446();
    }
}
