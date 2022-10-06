use ark_ec::{
    hashing::curve_maps::swu::SWUParams,
    models::{
        short_weierstrass::{Affine, SWCurveConfig},
        CurveConfig,
    },
};
use ark_ff::MontFp;

use crate::{Fq, Fq2, Fr};

type G2Affine = Affine<SwuIsoParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct SwuIsoParameters;

impl CurveConfig for SwuIsoParameters {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 7923214915284317143930293550643874566881017850177945424769256759165301436616933228209277966774092486467289478618404761412630691835764674559376407658497
    /// same as the original g2 curve
    /// sage: iso_G2.domain().order() == iso_G2.codomain().order()
    /// True
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x0000000000000001,
        0x452217cc90000000,
        0xa0f3622fba094800,
        0xd693e8c36676bd09,
        0x8c505634fae2e189,
        0xfbb36b00e1dcc40c,
        0xddd88d99a6f6a829,
        0x26ba558ae9562a,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 6764900296503390671038341982857278410319949526107311149686707033187604810669
    const COFACTOR_INV: Fr =
        MontFp!("6764900296503390671038341982857278410319949526107311149686707033187604810669");
}

// sage: E2p = iso_G2.domain()
// sage: r = 8444461749428370424248824938781546531375899335154063827935233455917409239041
// sage: E2p.order()/r
// 7923214915284317143930293550643874566881017850177945424769256759165301436616933228209277966774092486467289478618404761412630691835764674559376407658497
// sage: E2p
// Elliptic Curve defined by y^2 = x^3 +
// (69357795553467368835766998649443114298653120475771922004522583893765862042427351483161253261358624703462995261783*
// X2+203567575243095400658685394654545117908398249146024925306257919445062693445414588103741379252427065422417496933054)*
// x + (806998283981877041862626354975415285020485827233942100233224759047656510577433749137260740227904569833498998565*
// X2+249039961697346248294162904170316935273494032138504221215795383014884687447192317932476994472315647695087734549420)
// over Finite Field in X2 of size
// 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177^2
impl SWCurveConfig for SwuIsoParameters {
    /// COEFF_A =
    #[rustfmt::skip]
    const COEFF_A: Fq2 = Fq2::new(
                                    MontFp!("203567575243095400658685394654545117908398249146024925306257919445062693445414588103741379252427065422417496933054"),
                                    MontFp!("69357795553467368835766998649443114298653120475771922004522583893765862042427351483161253261358624703462995261783"),
    );

    /// COEFF_B =
    #[rustfmt::skip]
    const COEFF_B: Fq2 = Fq2::new(
        MontFp!("249039961697346248294162904170316935273494032138504221215795383014884687447192317932476994472315647695087734549420"),
        MontFp!("806998283981877041862626354975415285020485827233942100233224759047656510577433749137260740227904569833498998565"),
    );

    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);
}

const G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
const G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

// sage: G2_gen
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 +
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 44471777796618567688228760095584248343372454885978087674329841655595593880133139294404651664057692271364857231527
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 :
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 191377956145194479040228903677940355038998863371661730030204479850936075480341608934735952709786495341106477933498*
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 X2
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 +
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 115206687171448860889110309021279060303629519187879257051215751573842462972180856243991157572371972099444077110343
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 :
//(152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001*X2 1)
// sage: G2_gen *
// 8444461749428370424248824938781546531375899335154063827935233455917409239041
//(0 : 1 : 0)
/// G2_GENERATOR_X_C0 =
pub const G2_GENERATOR_X_C0: Fq = MontFp!("44471777796618567688228760095584248343372454885978087674329841655595593880133139294404651664057692271364857231527");

/// G2_GENERATOR_X_C1 =
pub const G2_GENERATOR_X_C1: Fq = MontFp!("152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001");

/// G2_GENERATOR_Y_C0 =
pub const G2_GENERATOR_Y_C0: Fq = MontFp!("115206687171448860889110309021279060303629519187879257051215751573842462972180856243991157572371972099444077110343");

/// G2_GENERATOR_Y_C1 =
pub const G2_GENERATOR_Y_C1: Fq = MontFp!("191377956145194479040228903677940355038998863371661730030204479850936075480341608934735952709786495341106477933498");

impl SWUParams for SwuIsoParameters {
    // sage: F2.primitive_element()
    // X2 + 12
    const ZETA: Fq2 = Fq2::new(MontFp!("12"), MontFp!("1")); // arbitatry primitive root of unity (element)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen() {
        let gen: G2Affine = SwuIsoParameters::GENERATOR;
        assert!(gen.is_on_curve());
        assert!(gen.is_in_correct_subgroup_assuming_on_curve());
    }
}
