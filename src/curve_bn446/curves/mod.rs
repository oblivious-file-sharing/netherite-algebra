use crate::curve_bn446::*;
use crate::endomorphisms::glv::BnGlvParameters;
use ark_ec::{
    bn,
    bn::{Bn, BnParameters, TwistType},
    ModelParameters,
};
use ark_ff::field_new;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub struct Parameters;

impl BnParameters for Parameters {
    const X: &'static [u64] = &[68719476737u64, 70368744177664u64];
    /// `x` is negative.
    const X_IS_NEGATIVE: bool = false;

    const ATE_LOOP_COUNT: &'static [i8] = &[
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1,
    ];

    /// Point Q x coordinate
    /// Recall that xi = u + 16, this is xi ^ ((q - 1) / 3)
    const TWIST_MUL_BY_Q_X: Fq2 = field_new!(
        Fq2,
        field_new!(
            Fq,
            "53558153621011479325426349315034270811125322046590322609779102882474382564304429471746703023140803971148477652553414598986280728688467"
        ),
        field_new!(
            Fq,
            "5393231721065334756362625835983113080313424024761601467014417219216618807287538532710945997317209942300317586393031070080437903353500"
        ),
    );
    // Point Q y coordinate
    /// Recall that xi = u + 16, this is xi ^ ((q - 1) / 2)
    const TWIST_MUL_BY_Q_Y: Fq2 = field_new!(
        Fq2,
        field_new!(
            Fq,
            "21887007078230277641290487545977304048816498067966680983946484526191854625712817027111151245647874878790862350313619445339436239665199"
        ),
        field_new!(
            Fq,
            "43557026439475285310086886777555239794145443705278081036303457819691464866819083503120171079672697756592696808936652336617039817572795"
        ),
    );
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Params = Fq2Parameters;
    type Fp6Params = Fq6Parameters;
    type Fp12Params = Fq12Parameters;
    type G1Parameters = g1::Parameters;
    type G2Parameters = g2::Parameters;
}

impl BnGlvParameters for Parameters {
    const BETA: <Self as BnParameters>::Fp = field_new!(Fq, "39370513046094214638711292860936120970669925098166405844133061722780077418403084822103936090966589486");
    const LAMBDA: <Self::G1Parameters as ModelParameters>::ScalarField = field_new!(Fr, "78741026092188429277422585721872211611399849651863042684662843239824929700335047920663405975411097661");
}

pub type Bn446 = Bn<Parameters>;

pub type G1Affine = bn::G1Affine<Parameters>;
pub type G1Projective = bn::G1Projective<Parameters>;
pub type G2Affine = bn::G2Affine<Parameters>;
pub type G2Projective = bn::G2Projective<Parameters>;
