use ark_ff::{biginteger::BigInteger256 as BigInteger, fields::*};

pub type Fr = Fp256<FrParameters>;

pub struct FrParameters;

impl Fp256Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 2;

    /// two_adic_root_of_unity = generator ^ r = 3614272690567954931885769240797874779046395893728115949589
    /// Written as  two_adic_root_of_unity * r (in the Montgomery representation)
    ///           = 8305812840650662635365060647846364983128330516745030972369075661190390837455
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        12517380601204666575u64,
        12489153293948124477u64,
        8319323408114352829u64,
        1323192325182062644u64,
    ]);
}
impl FpParameters for FrParameters {
    /// MODULUS = 16798108731015832284940804142231733909759579603404752749028378864165570215949
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        11601272640106397709u64,
        18419581738456973328u64,
        13417434401994702855u64,
        2676093114170474497u64,
    ]);

    const MODULUS_BITS: u32 = 254;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 2;

    /// R = pow(2, 256) % MODULUS
    ///   = 15003436851221201713926160155297504394712507045212047545287310822919708344242
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        4179340454199820210u64,
        162974011515469724u64,
        11729113956579540944u64,
        2390185388686704629u64,
    ]);

    /// R2 = R * R % MODULUS
    ///    = 16694411554397151134351660624246565287600790009138815106331186290317245196193
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        16106445354883000225u64,
        16179270253117906627u64,
        14051360972075784824u64,
        2659573200842625077u64,
    ]);

    /// INV = (-MODULUS) ^ {-1} % pow(2, 64) = 595423277050246629
    const INV: u64 = 16878105680422351163u64;

    /// GENERATOR = 2   (smallest generator)
    /// Written as 2 * R (in the Montgomery representation)
    ///          = 16772916675484418965367605628972582412446326874602633885791961452708694901496
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        11465225054178296568u64,
        11687352358820851180u64,
        11705771682239880432u64,
        2672079788181285086u64,
    ]);

    /// (MODULUS - 1) / 2 =
    /// 8399054365507916142470402071115866954879789801702376374514189432082785107974
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        5800636320053198854u64,
        18433162906083262472u64,
        15932089237852127235u64,
        1338046557085237248u64,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^s * t
    // Here, s = 2

    /// T = (MODULUS - 1) / 2^s =
    /// 4199527182753958071235201035557933477439894900851188187257094716041392553987
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        2900318160026599427u64,
        18439953489896407044u64,
        7966044618926063617u64,
        669023278542618624u64,
    ]);

    /// (T - 1) / 2 =
    /// 2099763591376979035617600517778966738719947450425594093628547358020696276993
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        1450159080013299713u64,
        18443348781802979330u64,
        3983022309463031808u64,
        334511639271309312u64,
    ]);
}
