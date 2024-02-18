use bech32::{ToBase32, Variant};
use cosmwasm_std::{ensure, to_json_string, Binary, CanonicalAddr, Deps, StdError};
use k256::sha2::{Digest, Sha256};
use ripemd::Ripemd160;

use crate::{msg::SignedData, state::PUBKEY};

// wallet address part before 1abc12345..., eg secret for secret1ac94abc....
const HRP: &str = "CHNGME";

pub fn verify_arbitrary(deps: Deps, data: &SignedData, signature: &[u8]) -> Result<(), StdError> {
    let key = PUBKEY.load(deps.storage);
    ensure!(key.is_ok(), StdError::generic_err("Key Not Set"));
    let key = key.unwrap();

    let digest = Sha256::new_with_prefix(generate_amino_transaction_string(
        pubkey_to_account(&key, HRP).as_str(),
        to_json_string(&data)?.as_str(),
    ))
    .finalize();

    deps.api.secp256k1_verify(&digest, signature, &key)?;

    Ok(())
}

fn generate_amino_transaction_string(signer: &str, data: &str) -> String {
    format!(
        "{{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{{\"amount\":[],\"gas\":\"0\"}},\"memo\":\"\",\"msgs\":[{{\"type\":\"sign/MsgSignData\",\"value\":{{\"data\":\"{}\",\"signer\":\"{}\"}}}}],\"sequence\":\"0\"}}",
        data, signer
    )
}

pub fn pubkey_to_account(pubkey: &Binary, hrp: &str) -> String {
    let base32_addr = pubkey_to_canonical(pubkey).0.as_slice().to_base32();
    let account: String = bech32::encode(hrp, base32_addr, Variant::Bech32).unwrap();
    account
}

fn pubkey_to_canonical(pubkey: &Binary) -> CanonicalAddr {
    let mut hasher = Ripemd160::new();
    hasher.update(sha_256(&pubkey.0));
    CanonicalAddr(Binary(hasher.finalize().to_vec()))
}

fn sha_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(hash.as_slice());
    result
}
