use ark_ff::{biginteger::BigInteger256 as BigInteger, fields::*};

pub type Fq = Fp256<FqParameters>;

pub struct FqParameters;

impl Fp256Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 1;

    /// two_adic_root_of_unity = generator ^ r = 16798108731015832284940804142231733909889187121439069848933715426072753864722
    /// Written as  two_adic_root_of_unity * r (in the Montgomery representation)
    ///           = 1794671879794630571014643986934229515954325184432924903078423974596147413125
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        10448351135499550853u64,
        12098638923954126985u64,
        1688320445415161914u64,
        285907725483769868u64,
    ]);
}
impl FpParameters for FqParameters {
    /// MODULUS = 16798108731015832284940804142231733909889187121439069848933715426072753864723
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        12033618204333965331u64,
        6998875295910461459u64,
        13417434401994702856u64,
        2676093114170474497u64,
    ]);

    const MODULUS_BITS: u32 = 254;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 2;

    /// R = pow(2, 256) % MODULUS
    ///   = 15003436851221201713926160155297504393934861937006144945855291451476606451598
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        1585267068834414478u64,
        13346980445665886090u64,
        11729113956579540941u64,
        2390185388686704629u64,
    ]);

    /// R2 = R * R % MODULUS
    ///    = 12230467316513068056352497508934426021303424122708754667510789219591570605885
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        12963759361560168253u64,
        6192378494175724529u64,
        2890811900596491599u64,
        1948425855130679869u64,
    ]);

    /// INV = (-MODULUS) ^ {-1} % pow(2, 64) = 595423277050246629
    const INV: u64 = 595423277050246629u64;

    /// GENERATOR = 3   (smallest generator)
    /// Written as 3 * R (in the Montgomery representation)
    ///          = 11414093091631940571896872181429045362026211568140295139698443502284311625348
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        17582052945254416004u64,
        7596446671467183734u64,
        8352473065749217112u64,
        1818369937719164893u64,
    ]);

    /// (MODULUS - 1) / 2 =
    /// 8399054365507916142470402071115866954944593560719534924466857713036376932361
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        15240181139021758473u64,
        3499437647955230729u64,
        15932089237852127236u64,
        1338046557085237248u64,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^s * t
    // Here, s = 1, or essentially with the minimal 2-arity

    /// T = (MODULUS - 1) / 2^s =
    /// 8399054365507916142470402071115866954944593560719534924466857713036376932361
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        15240181139021758473u64,
        3499437647955230729u64,
        15932089237852127236u64,
        1338046557085237248u64,
    ]);

    /// (T - 1) / 2 =
    /// 4199527182753958071235201035557933477472296780359767462233428856518188466180
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        16843462606365655044u64,
        1749718823977615364u64,
        7966044618926063618u64,
        669023278542618624u64,
    ]);
}

#[allow(dead_code)]
pub const FQ_ONE: Fq = Fq::new(FqParameters::R);
#[allow(dead_code)]
pub const FQ_ZERO: Fq = Fq::new(BigInteger([0, 0, 0, 0]));
