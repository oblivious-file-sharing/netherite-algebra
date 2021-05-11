use super::*;
use ark_ff::{field_new, fields::*};

pub type Fq12 = Fp12<Fq12Parameters>;

#[derive(Clone, Copy)]
pub struct Fq12Parameters;

impl Fp12Parameters for Fq12Parameters {
    type Fp6Params = Fq6Parameters;

    // See the documentation of `Fp12Parameters` on why using this as nonresidue
    const NONRESIDUE: Fq6 = field_new!(Fq6, FQ2_ZERO, FQ2_ONE, FQ2_ZERO);

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "1"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "12310438583873020660552735091161044116898065562217439662059245424880585960937"),
            field_new!(Fq, "4487670147142811624388069051070689792991121559221630186874470001192167903786"),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "1807136345283977465813277102364620289631804529403213381640"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "571759232279933406358082416437452803737039382138144404732186496954757578318"),
            field_new!(Fq, "16226349498735898878582721725794281106152147739300925444201528929117996286405"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "1807136345283977465813277102364620289631804529403213381639"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "5059429379422745030746151467508142596728160941359774591606656498146925482104"),
            field_new!(Fq, "11738679351593087254194652674723591313161026180079295257327058927925828382619"),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832284940804142231733909889187121439069848933715426072753864722"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "4487670147142811624388069051070689792991121559221630186874470001192167903786"),
            field_new!(Fq, "12310438583873020660552735091161044116898065562217439662059245424880585960937"),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832283133667796947756444075910019074449559301910896669540483083"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "16226349498735898878582721725794281106152147739300925444201528929117996286405"),
            field_new!(Fq, "571759232279933406358082416437452803737039382138144404732186496954757578318"),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832283133667796947756444075910019074449559301910896669540483084"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "11738679351593087254194652674723591313161026180079295257327058927925828382619"),
            field_new!(Fq, "5059429379422745030746151467508142596728160941359774591606656498146925482104"),
        ),
    ];
}
