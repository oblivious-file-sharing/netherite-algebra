use super::*;
use ark_ff::{field_new, fields::*};

pub type Fq6 = Fp6<Fq6Parameters>;

#[derive(Clone, Copy)]
pub struct Fq6Parameters;

// The extension is done using the irreducible polynomial v ^ 3 - u - 1 over Fq2

impl Fp6Parameters for Fq6Parameters {
    type Fp2Params = Fq2Parameters;

    /// NONRESIDUE = U + 1
    #[rustfmt::skip]
    const NONRESIDUE: Fq2 = field_new!(Fq2, FQ_ONE, FQ_ONE);

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
        field_new!(Fq2, FQ_ONE, FQ_ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
        field_new!(Fq2,
            FQ_ZERO,
            field_new!(Fq, "16798108731015832283133667796947756444075910019074449559301910896669540483083"),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
        field_new!(Fq2,
            field_new!(Fq, "1807136345283977465813277102364620289631804529403213381639"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
        field_new!(Fq2, FQ_ZERO, FQ_ONE),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832283133667796947756444075910019074449559301910896669540483083"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
        field_new!(Fq2,
            FQ_ZERO,
            field_new!(Fq, "1807136345283977465813277102364620289631804529403213381639"),
        ),
    ];
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^((2*(q^0) - 2) / 3)
        field_new!(Fq2, FQ_ONE, FQ_ZERO),
        // Fp2::NONRESIDUE^((2*(q^1) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832283133667796947756444075910019074449559301910896669540483084"),
            FQ_ZERO
        ),
        // Fp2::NONRESIDUE^((2*(q^2) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832283133667796947756444075910019074449559301910896669540483083"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^3) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "16798108731015832284940804142231733909889187121439069848933715426072753864722"),
            FQ_ZERO),
        // Fp2::NONRESIDUE^((2*(q^4) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "1807136345283977465813277102364620289631804529403213381639"),
            FQ_ZERO,
        ),
        // Fp2::NONRESIDUE^((2*(q^5) - 2) / 3)
        field_new!(Fq2,
            field_new!(Fq, "1807136345283977465813277102364620289631804529403213381640"),
            FQ_ZERO,
        ),
    ];

    #[inline(always)]
    fn mul_fp2_by_nonresidue(fe: &Fq2) -> Fq2 {
        //   (c0 + u * c1) * (u + 1)
        // = c0 * u - c1 + c0 + u * c1
        // = (c0 + c1) * u + (c0 - c1)

        let c0 = fe.c0 - fe.c1;
        let c1 = fe.c0 + fe.c1;
        field_new!(Fq2, c0, c1)
    }
}
