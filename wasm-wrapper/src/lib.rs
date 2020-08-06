#[macro_use]
extern crate serde_derive;

use rand::Rng;

use powersoftau::{
    cli_common::*,
    keypair::keypair,
    parameters::{CeremonyParams, CheckForCorrectness},
    BatchedAccumulator,
};
use snark_utils::{calculate_hash, get_rng, user_system_randomness, UseCompression};

use zexe_algebra::{Bls12_377, Bls12_381, PairingEngine as Engine, BW6_761};

use wasm_bindgen::prelude::*;

const INPUT_IS_COMPRESSED: UseCompression = UseCompression::No;
const COMPRESS_THE_OUTPUT: UseCompression = UseCompression::Yes;
const CHECK_INPUT_CORRECTNESS: CheckForCorrectness = CheckForCorrectness::No;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct ContributionResponse {
    current_accumulator_hash: Vec<u8>,
    response: Vec<u8>,
    contribution_hash: Vec<u8>,
}

#[wasm_bindgen]
pub struct Powersoftau {}

#[wasm_bindgen]
impl Powersoftau {
    pub fn contribute(curve_kind: &str, batch_size: usize, power: usize, challenge: &[u8]) -> Result<JsValue, JsValue> {
        let rng = get_rng(&user_system_randomness());
        let res = match curve_from_str(curve_kind).expect("invalid curve_kind") {
            CurveKind::Bls12_381 => {
                contribute_challenge(&challenge, &get_parameters::<Bls12_381>(batch_size, power), rng)
            }
            CurveKind::Bls12_377 => {
                contribute_challenge(&challenge, &get_parameters::<Bls12_377>(batch_size, power), rng)
            }
            CurveKind::BW6 => contribute_challenge(&challenge, &get_parameters::<BW6_761>(batch_size, power), rng),
        };
        return Ok(JsValue::from_serde(&res.ok().unwrap()).unwrap());
    }
}

pub fn get_parameters<E: Engine>(batch_size: usize, power: usize) -> CeremonyParams<E> {
    CeremonyParams::<E>::new(power, batch_size)
}

pub fn contribute_challenge<T: Engine + Sync>(
    challenge: &[u8],
    parameters: &CeremonyParams<T>,
    mut rng: impl Rng,
) -> Result<ContributionResponse, String> {
    let expected_challenge_length = match INPUT_IS_COMPRESSED {
        UseCompression::Yes => parameters.contribution_size,
        UseCompression::No => parameters.accumulator_size,
    };

    if challenge.len() != expected_challenge_length {
        return Err(format!(
            "The size of challenge file should be {}, but it's {}, so something isn't right.",
            expected_challenge_length,
            challenge.len()
        ));
    }

    let required_output_length = match COMPRESS_THE_OUTPUT {
        UseCompression::Yes => parameters.contribution_size,
        UseCompression::No => parameters.accumulator_size + parameters.public_key_size,
    };

    let mut response: Vec<u8> = vec![];
    let current_accumulator_hash = calculate_hash(&challenge);

    for i in 0..required_output_length {
        response.push(current_accumulator_hash[i % current_accumulator_hash.len()]);
    }

    // Construct our keypair using the RNG we created above
    let (pubkey, privkey) = match keypair(&mut rng, current_accumulator_hash.as_ref()) {
        Ok(pair) => pair,
        Err(_) => return Err("could not generate keypair".to_string()),
    };

    // this computes a transformation and writes it
    match BatchedAccumulator::contribute(
        &challenge,
        &mut response,
        INPUT_IS_COMPRESSED,
        COMPRESS_THE_OUTPUT,
        CHECK_INPUT_CORRECTNESS,
        &privkey,
        &parameters,
    ) {
        Ok(_) => match pubkey.write(&mut response, COMPRESS_THE_OUTPUT, &parameters) {
            Ok(_) => {
                let contribution_hash = calculate_hash(&response);

                return Ok(ContributionResponse {
                    current_accumulator_hash: current_accumulator_hash.as_slice().iter().cloned().collect(),
                    response,
                    contribution_hash: contribution_hash.as_slice().iter().cloned().collect(),
                });
            }
            Err(_) => {
                return Err("unable to write public key".to_string());
            }
        },
        Err(_) => {
            return Err("must contribute with the key".to_string());
        }
    }
}
