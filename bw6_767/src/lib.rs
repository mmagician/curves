#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the BW6_767 curve generated in [\[EG20\]](https://eprint.iacr.org/2020/351).
//! The name denotes that it is a curve generated using the Brezing--Weng
//! method, and that its embedding degree is 6.
//! The main feature of this curve is that the scalar field equals the base
//! field of the BLS12_381 curve.
//!
//! Curve information:
//! * Base field: q = 496597749679620867773432037469214230242402307330180853437434581099336634619713640485778675608223760166307530047354464605410050411581079376994803852937842168733702867087556948851016246640584660942486895230518034810309227309966899431
//! * Scalar field: r = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 46
//!
//! G1 curve equation: y^2 = x^3 + Ax + B, where
//! * A = 0,
//! * B = 1
//!
//! G2 curve equation: y^2 = x^3 + Ax + B, where
//! * A = 0
//! * B = 3

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
