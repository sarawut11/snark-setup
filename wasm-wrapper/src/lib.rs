mod utils;

use powersoftau::{cli_common::*, parameters::CeremonyParams};

use zexe_algebra::{Bls12_377, Bls12_381, PairingEngine as Engine, BW6_761};

use wasm_bindgen::prelude::*;

type Error = String;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Powersoftau {}

#[wasm_bindgen]
impl Powersoftau {
    pub fn new(challenge_filename: &str, curve_kind: &str, batch_size: usize, power: usize) {
        match convert_curve_kind(curve_kind).expect("invalid curve kind") {
            CurveKind::Bls12_381 => new_challenge(challenge_filename, &get_parameters::<Bls12_381>(batch_size, power)),
            CurveKind::Bls12_377 => new_challenge(challenge_filename, &get_parameters::<Bls12_377>(batch_size, power)),
            CurveKind::BW6 => new_challenge(challenge_filename, &get_parameters::<BW6_761>(batch_size, power)),
        };
    }
}

pub fn get_parameters<E: Engine>(batch_size: usize, power: usize) -> CeremonyParams<E> {
    CeremonyParams::<E>::new(power, batch_size)
}

pub fn convert_curve_kind(curve_kind: &str) -> Result<CurveKind, Error> {
    match curve_kind {
        "Bls12_381" => Ok(CurveKind::Bls12_381),
        "Bls12_377" => Ok(CurveKind::Bls12_377),
        "BW6" => Ok(CurveKind::BW6),
        _ => Err("error".to_string()),
    }
}
