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
    pub fn new(curve_kind: &str, batch_size: usize, power: usize) {
        match curve_from_str(curve_kind) {
            Ok(curve) => {
                // match curve {
                //     CurveKind::Bls12_381 => new_challenge(&get_parameters::<Bls12_381>(batch_size, power)),
                //     CurveKind::Bls12_377 => new_challenge(&get_parameters::<Bls12_377>(batch_size, power)),
                //     CurveKind::BW6 => new_challenge(&get_parameters::<BW6_761>(batch_size, power)),
                // };
            }
            Err(err) => {}
        }
    }
}

// pub fn get_parameters<E: Engine>(batch_size: usize, power: usize) -> CeremonyParams<E> {
//     CeremonyParams::<E>::new(power, batch_size)
// }

// pub fn new_challenge<T: Engine + Sync>(parameters: &CeremonyParams<T>) {
// Write a blank BLAKE2b hash:
// let hash = blank_hash();
// (&mut writable_map[0..])
//     .write_all(hash.as_slice())
//     .expect("unable to write a default hash to mmap");
// writable_map
//     .flush()
//     .expect("unable to write blank hash to challenge file");

// println!("Blank hash for an empty challenge:");
// print_hash(&hash);

// BatchedAccumulator::generate_initial(&mut writable_map, COMPRESS_NEW_CHALLENGE, &parameters)
//     .expect("generation of initial accumulator is successful");
// writable_map.flush().expect("unable to flush memmap to disk");

// // Get the hash of the contribution, so the user can compare later
// let output_readonly = writable_map.make_read_only().expect("must make a map readonly");
// let contribution_hash = calculate_hash(&output_readonly);

// println!("Empty contribution is formed with a hash:");
// print_hash(&contribution_hash);
// println!("Wrote a fresh accumulator to challenge file");
// }
