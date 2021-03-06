use powersoftau::parameters::CeremonyParams;
use snark_utils::UseCompression;
use test_helpers::setup_verify;

use snarkos_curves::{bls12_377::Bls12_377 as AleoBls12_377, bw6_761::BW6_761 as AleoBW6};
use snarkos_models::curves::{AffineCurve as AleoAffineCurve, PairingEngine as AleoPairingEngine};
use snarkos_utilities::serialize::{
    CanonicalDeserialize as AleoCanonicalDeserialize,
    ConstantSerializedSize as AleoConstantSerializedSize,
};

use zexe_algebra::{
    AffineCurve as ZexeAffineCurve,
    Bls12_377 as ZexeBls12_377,
    PairingEngine as ZexePairingEngine,
    BW6_761 as ZexeBW6,
};
use zexe_algebra_core::serialize::{
    CanonicalDeserialize as ZexeCanonicalDeserialize,
    ConstantSerializedSize as ZexeConstantSerializedSize,
};

use std::io::Read;

fn compatible_powersoftau<Aleo: AleoPairingEngine, Zexe: ZexePairingEngine>() -> anyhow::Result<()> {
    // Generate an accumulator via Zexe's trusted setup.
    let (powers, batch) = (6, 4);
    let params = CeremonyParams::<Zexe>::new(powers, batch);

    // Perform 1 power of tau contribution (assume Powers of Tau is computed correctly)
    let compressed = UseCompression::No;
    let (_, output, _, _) = setup_verify(compressed, compressed, &params);

    // Advance the cursor past the output hash.
    let mut reader = std::io::BufReader::new(std::io::Cursor::new(output));
    reader.read_exact(&mut vec![0; params.hash_size])?;

    // Verify that the outputs are equivalent.

    fn assert_compatibility<Aleo: AleoAffineCurve, Zexe: ZexeAffineCurve, R: Read>(
        reader: &mut R,
    ) -> anyhow::Result<()> {
        let mut buffer = vec![0; Aleo::UNCOMPRESSED_SIZE];
        reader.read_exact(&mut buffer)?;

        let aleo_group: Aleo = AleoCanonicalDeserialize::deserialize_uncompressed(&mut std::io::Cursor::new(&buffer))?;
        let mut aleo = vec![];
        aleo_group.write(&mut aleo)?;

        let zexe_group: Zexe = ZexeCanonicalDeserialize::deserialize_uncompressed(&mut std::io::Cursor::new(&buffer))?;
        let mut zexe = vec![];
        zexe_group.write(&mut zexe)?;

        assert_eq!(aleo, zexe);
        Ok(())
    }

    assert_eq!(Aleo::G1Affine::UNCOMPRESSED_SIZE, Zexe::G1Affine::UNCOMPRESSED_SIZE);
    assert_eq!(Aleo::G2Affine::UNCOMPRESSED_SIZE, Zexe::G2Affine::UNCOMPRESSED_SIZE);

    for _ in 0..params.powers_g1_length {
        assert_compatibility::<Aleo::G1Affine, Zexe::G1Affine, _>(&mut reader)?;
    }
    for _ in 0..params.powers_length {
        assert_compatibility::<Aleo::G2Affine, Zexe::G2Affine, _>(&mut reader)?;
    }
    for _ in 0..params.powers_length {
        assert_compatibility::<Aleo::G1Affine, Zexe::G1Affine, _>(&mut reader)?;
    }
    for _ in 0..params.powers_length {
        assert_compatibility::<Aleo::G1Affine, Zexe::G1Affine, _>(&mut reader)?;
    }
    assert_compatibility::<Aleo::G2Affine, Zexe::G2Affine, _>(&mut reader)?;

    Ok(())
}

#[test]
fn test_aleo_zexe_bls12_377_compatibility() {
    compatible_powersoftau::<AleoBls12_377, ZexeBls12_377>().unwrap();
}

#[test]
fn test_aleo_zexe_bw6_761_compatibility() {
    compatible_powersoftau::<AleoBW6, ZexeBW6>().unwrap();
}
