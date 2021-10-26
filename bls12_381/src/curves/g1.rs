use crate::*;
use ark_ec::{
    bls12,
    bls12::Bls12Parameters,
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::GroupAffine,
    AffineCurve, ProjectiveCurve,
};
use ark_ff::{biginteger::BigInteger256, field_new, Zero};
use ark_std::ops::Neg;
use ark_ec::hashing::curve_maps::wb::WBParams;

use super::g1_swu_iso::Parameters as SWUIsogenousCurveParameters;

pub type G1Affine = bls12::G1Affine<crate::Parameters>;
pub type G1Projective = bls12::G1Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 4
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "4");

    /// COFACTOR = (x - 1)^2 / 3  = 76329603384216526031706109802092473003
    const COFACTOR: &'static [u64] = &[0x8c00aaab0000aaab, 0x396c8c005555e156];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 52435875175126190458656871551744051925719901746859129887267498875565241663483
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "52435875175126190458656871551744051925719901746859129887267498875565241663483");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve(p: &GroupAffine<Parameters>) -> bool {
        // Algorithm from Section 6 of https://eprint.iacr.org/2021/1130.
        //
        // Check that endomorphism_p(P) == -[X^2]P

        let x = BigInteger256::new([crate::Parameters::X[0], 0, 0, 0]);

        // An early-out optimization described in Section 6.
        // If uP == P but P != point of infinity, then the point is not in the right subgroup.
        let x_times_p = p.mul(x);
        if x_times_p.eq(p) && !p.infinity {
            return false;
        }

        let minus_x_squared_times_p = x_times_p.mul(x).neg();
        let endomorphism_p = endomorphism(p);
        minus_x_squared_times_p.eq(&endomorphism_p)
    }
}

/// G1_GENERATOR_X =
/// 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507");

/// G1_GENERATOR_Y =
/// 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569");

/// BETA is a non-trivial cubic root of unity in Fq.
pub const BETA: Fq = field_new!(Fq, "793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350");

pub fn endomorphism(p: &GroupAffine<Parameters>) -> GroupAffine<Parameters> {
    // Endomorphism of the points on the curve.
    // endomorphism_p(x,y) = (BETA * x, y) where BETA is a non-trivial cubic root of unity in Fq.
    let mut res = (*p).clone();
    res.x *= BETA;
    res
}

impl WBParams for Parameters
{
    type IsogenousCurve = SWUIsogenousCurveParameters;

    const PHI_X_NOM: &'static [<Self::IsogenousCurve as ModelParameters>::BaseField] = &[field_new!(Fq, "1058488477413994682556770863004536636444795456512795473806825292198091015005841418695586811009326456605062948114985"), field_new!(Fq, "2756657124183929639940341559332863560985099912924783743137983687385942647530234634138642360072966950403354118194880"), field_new!(Fq, "698396551686164607792478797181335970223204059946034999723234339823539961139901150842584755596191372859677741313422"), field_new!(Fq, "1239271775787030039269460763652455868148971086016832054354147730155061349388626624328773377658494412538595239256855"), field_new!(Fq, "2458049485161426253398308320890830930555526088324701597510592431647721369610314802890725095474874074634194669518436"), field_new!(Fq, "248188830067800966021611907001049410443171766148926993624301072093542166689921157756350157715209127315556469919811"), field_new!(Fq, "3415427104483187489859740871640064348492611444552862448295571438270821994900526625562705192993481400731539293415811"), field_new!(Fq, "1424741831220874356476333227468129624471472782807764018784263716426284995285578915327628560152704910696985638070031"), field_new!(Fq, "61316326124367244515865706164471217084261738749879925739220878889304439271692421994859529859373651892126645952478"), field_new!(Fq, "2051387046688339481714726479723076305756384619135044672831882917686431912682625619320120082313093891743187631791280"), field_new!(Fq, "1582172990273894682725716146256297593851554078446457591584154223376480866715343525953458444978680215440412886996200"), field_new!(Fq, "3761822637321485742094536206199512035685972329360337092690555528605752326213440950527352563934445837165125977345128")];
                                                                                               

    const PHI_X_DEN: &'static [<Self::IsogenousCurve as ModelParameters>::BaseField] = &[field_new!(Fq, "1"), field_new!(Fq, "1355518942857092779104773143196445884975815408961178437135200875404433360418847982032652351120700883660623679118159"), field_new!(Fq, "684141363729077267665842443966270070725528746651574246973574894998264196269884726340959705960779078660850485681497"), field_new!(Fq, "3179090966864399634396993677377903383656908036827452986467581478509513058347781039562481806409014718357094150199902"), field_new!(Fq, "3729460208364303450659860043245117836792062818643913458168515950720008978604972045673944667221729675671707946923021"), field_new!(Fq, "2787329879967655380381218769366715121683732401785552165471280254930041329235866427760690206934082971711204373036769"), field_new!(Fq, "3025903087998593826923738290305187197829899948335370692927241015584233559365859980023579293766193297662657497834014"), field_new!(Fq, "2439329436782028399166625708751323253248871941520474623095864400521929929188290312899217468935910181179336068540275"), field_new!(Fq, "610552060666338680048265043345785271200283516960578188721887711701602621050421759883463448407237338290466946893545"), field_new!(Fq, "2822220997908397120956501031591772354860004534930174057793539372552395729721474912921980407622851861692773516917759"), field_new!(Fq, "3949438676361386880769263910006560135979986067074971624024178233787093666769860448538216069627988502376148329127381")];

    const PHI_Y_NOM: &'static [<Self::IsogenousCurve as ModelParameters>::BaseField] = &[field_new!(Fq, "3370924952219000111210625390420697640496067348723987858345031683392215988129398381698161406651860675722373763741188"), field_new!(Fq, "2922895688438869655803185556286420403397802691723657346548307498540648684066474272936979722182960006715481007746439"), field_new!(Fq, "1967501030954114997918812128990512798136077044765138085902549248681331523022349398587390395919802044865687181234417"), field_new!(Fq, "1707589313757812493102695021134258021969283151093981498394095062397393499601961942449581422761005023512037430861560"), field_new!(Fq, "3961890550435212003631878720741046102282781357458395684082291998816751808643768526367549852177588213715211235373916"), field_new!(Fq, "2092842804947501956457889650155528390118620253793900717919970123642236515732658355049092877407901587228724229223110"), field_new!(Fq, "2171468288973248519912068884667133903101171670397991979582205855298465414047741472281361964966463442016062407908400"), field_new!(Fq, "331351138484847160772695176511692246547145983117580056167533060034011313825061073939286437522432693033324549699722"), field_new!(Fq, "3406136881215263818934773248465685844790898101337646888092845387936582793628141067747381197872508534868489799639699"), field_new!(Fq, "718493410301850496156792713845282235942975872282052335612908458061560958159410402177452633054233549648465863759602"), field_new!(Fq, "889147988023366972622306341891649433228352963186679791369365907311293432508530696403569262531255821940400079347315"), field_new!(Fq, "3668073836642475590306080768959183669825119857553168245915502992504527111582288680993272182771099907781295865642364"), field_new!(Fq, "303251954782077855462083823228569901064301365507057490567314302006681283228886645653148231378803311079384246777035"), field_new!(Fq, "3614401657395238041642341964430562932885284129837437306094802414995585690467848276262207178353676484330879154111757"), field_new!(Fq, "1511190695657960398963160955727174407082178148587899660611396357887273149842318573217989398009716786569780748006283"), field_new!(Fq, "1393399195776646641963150658816615410692049723305861307490980409834842911816308830479576739332720113414154429643571")];
    
    const PHI_Y_DEN: &'static [<Self::IsogenousCurve as ModelParameters>::BaseField] = &[field_new!(Fq, "1"), field_new!(Fq, "32073636674805471948264801926716749185281703472263713036772245044634215382853040827634712116543493471988382397345"), field_new!(Fq, "939263815598411948154903329612610969504805630387130456691588334780964163089645546614263028349325528011160458395367"), field_new!(Fq, "1668238650112823419388205992952852912407572045257706138925379268508860023191233729074751042562151098884528280913356"), field_new!(Fq, "2381968764806454673720955440620228057919683152985846777578004069901421179563597288802750793515266450448328892335136"), field_new!(Fq, "3898950452492751420431682235553362135160293985934212849206350896691847297403795877806915602445207834970900272834330"), field_new!(Fq, "3459661102222301807083870307127272890283709299202626530836335779816726101522661683404130556379097384249447658110805"), field_new!(Fq, "2309935917761931164183799788994195243166135866798971442738101234750394172839060646960604788222384454747599044244610"), field_new!(Fq, "1189518655724056699355159955938426141594501095232997073362878098547301776615800227099534760471261853454585261625212"), field_new!(Fq, "3443845896188810583748698342858554856823966611538932245284665132724280883115455093457486044009395063504744802318172"), field_new!(Fq, "3838344850882296394939911407571260624820518547352197287854517462270146783414326660145944161947898461843919024387456"), field_new!(Fq, "3819161294135653749488194485080848928281288158623143986707267064691105653307717275066596854814679462226419882338445"), field_new!(Fq, "3496628876382137961119423566187258795236027183112131017519536056628828830323846696121917502443333849318934945158166"), field_new!(Fq, "3115801080798375384198119675359305198682407447857559059827677597200837822471239996878963936531572341910295439078930"), field_new!(Fq, "3955937245707125245654875920344947126613343076099634862876498453376019064171085472617926084668015244086355452990926"), field_new!(Fq, "3396434800020507717552209507749485772788165484415495716688989613875369612529138640646200921379825018840894888371137")];

}
