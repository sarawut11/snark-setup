#[macro_use]
extern crate serde_derive;

mod utils;

use powersoftau::{cli_common::*, parameters::CeremonyParams, BatchedAccumulator};
use snark_utils::{blank_hash, calculate_hash, UseCompression};

use zexe_algebra::{Bls12_377, Bls12_381, PairingEngine as Engine, BW6_761};

use wasm_bindgen::prelude::*;

const COMPRESS_NEW_CHALLENGE: UseCompression = UseCompression::No;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct ChallengeInfo {
    challenge: Vec<u8>,
    hash: Vec<u8>,
}

#[wasm_bindgen]
pub struct Powersoftau {}

#[wasm_bindgen(catch)]
impl Powersoftau {
    pub fn new(curve_kind: &str, batch_size: usize, power: usize) -> Result<JsValue, JsValue> {
        let mut res = match curve_from_str(curve_kind).expect("123") {
            CurveKind::Bls12_381 => new_challenge(&get_parameters::<Bls12_381>(batch_size, power)),
            CurveKind::Bls12_377 => new_challenge(&get_parameters::<Bls12_377>(batch_size, power)),
            CurveKind::BW6 => new_challenge(&get_parameters::<BW6_761>(batch_size, power)),
        };
        return Ok(JsValue::from_serde(&res.ok().unwrap()).unwrap());
    }
}

pub fn get_parameters<E: Engine>(batch_size: usize, power: usize) -> CeremonyParams<E> {
    CeremonyParams::<E>::new(power, batch_size)
}

pub fn new_challenge<T: Engine + Sync>(parameters: &CeremonyParams<T>) -> Result<ChallengeInfo, String> {
    let expected_challenge_length = match COMPRESS_NEW_CHALLENGE {
        UseCompression::Yes => parameters.contribution_size - parameters.public_key_size,
        UseCompression::No => parameters.accumulator_size,
    };

    let mut writable_map: Vec<u8> = vec![];
    // Write a blank BLAKE2b hash:
    let hash = blank_hash();

    for i in 0..expected_challenge_length {
        writable_map.push(hash[i % hash.len()]);
    }

    match BatchedAccumulator::generate_initial(&mut writable_map, COMPRESS_NEW_CHALLENGE, &parameters) {
        Ok(_) => {
            // Get the hash of the contribution, so the user can compare later
            let contribution_hash = calculate_hash(&writable_map);
            return Ok(ChallengeInfo {
                challenge: writable_map,
                hash: contribution_hash.as_slice().iter().cloned().collect(),
            });
        }
        Err(_) => {
            return Err("generation of initial accumulator is successful".to_string());
        }
    };
}
