use anyhow::{Context, Result};
use core::slice;
use pgp::bytes::Bytes;
use pgp::composed::{
    Deserializable, LiteralDataReader, Message, MessageBuilder, SignedPublicKey, SignedSecretKey,
};
use pgp::crypto::hash::HashAlgorithm;
use pgp::crypto::sym::SymmetricKeyAlgorithm;
use pgp::packet::LiteralData;
use pgp::types::{EskType, Password, PkeskBytes, PublicKeyTrait, SecretKeyTrait, SignatureBytes};
use rand::{prelude::*, CryptoRng, Rng};
use std::{fs, io::Cursor, path::Path};

pub fn read_skey_file(path: impl AsRef<Path>) -> Result<SignedSecretKey> {
    let bytes = fs::read(path.as_ref())
        .with_context(|| format!("Failed to read pgp secret key at {:?}", path.as_ref()))?;

    let (skey, _) = SignedSecretKey::from_armor_single_buf(Cursor::new(bytes))
        .with_context(|| format!("Failed to parse pgp secret key at {:?}", path.as_ref()))?;

    Ok(skey)
}

fn read_pkey_file(path: impl AsRef<Path>) -> Result<SignedPublicKey> {
    let bytes = fs::read(path.as_ref())
        .with_context(|| format!("Failed to read pgp public key at {:?}", path.as_ref()))?;
    let (pkey, _) = SignedPublicKey::from_armor_single_buf(Cursor::new(bytes))
        .with_context(|| format!("Failed to parse pgp public key at {:?}", path.as_ref()))?;
    Ok(pkey)
}

pub fn build_signed_message(
    key: &SignedSecretKey,
    plaintext: &[u8],
    rng: &mut (impl Rng + CryptoRng),
    hash_alg: HashAlgorithm,
) -> Result<Vec<u8>> {
    let mut builder = MessageBuilder::from_bytes("", plaintext.to_vec());

    builder.sign(&key.primary_key, Password::empty(), hash_alg);
    let bytes = builder.to_vec(rng)?;
    Ok(bytes)
}

fn decode_signed_message(signed_message: &[u8], key: &SignedPublicKey) -> Result<Vec<u8>> {
    let mut message = Message::from_bytes(Cursor::new(signed_message))?;
    let inner = message.as_data_vec()?;
    message.verify(key)?;
    Ok(inner)
}

fn build_encrypted_message(
    encrypting: &SignedPublicKey,
    plaintext: &[u8],
    rng: &mut (impl Rng + CryptoRng),
    sym_alg: SymmetricKeyAlgorithm,
) -> Result<Vec<u8>> {
    let builder = MessageBuilder::from_bytes("", plaintext.to_vec());
    let mut builder = builder.seipd_v1(&mut *rng, sym_alg);

    assert!(!encrypting.is_encryption_key());
    let subkey = encrypting
        .public_subkeys
        .iter()
        .find(|key| key.is_encryption_key())
        .unwrap();

    builder.encrypt_to_key(&mut *rng, subkey)?;
    let bytes = builder.to_vec(rng)?;
    Ok(bytes)
}

fn decode_encrypted_message(key: &SignedSecretKey, message: &[u8]) -> Result<Vec<u8>> {
    let message = Message::from_bytes(Cursor::new(message))?;
    let mut message = message.decrypt(&Password::empty(), key)?;
    Ok(message.as_data_vec()?)
}

fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() -> Result<()> {
        let pkey =
            read_pkey_file("test.asc").with_context(|| "Must create a file called test.asc")?;
        let skey = read_skey_file("test_secret.asc")
            .with_context(|| "Must create a file called test_secret.asc")?;

        let plaintext = b"hello world";
        let mut rng = thread_rng();
        let hash_alg = HashAlgorithm::Sha256;

        let signed_text = build_signed_message(&skey, plaintext, &mut rng, hash_alg)
            .with_context(|| "Signing message")?;
        assert_eq!(
            decode_signed_message(&signed_text, &pkey).with_context(|| "Verifying message")?,
            plaintext
        );
        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt() -> Result<()> {
        let pkey = read_pkey_file("test.asc")?;
        let skey = read_skey_file("test_secret.asc")?;

        let plaintext = b"hello world";
        let mut rng = thread_rng();
        let sym_alg = SymmetricKeyAlgorithm::AES256;

        let signed_text = build_encrypted_message(&pkey, plaintext, &mut rng, sym_alg)
            .with_context(|| "Encrypting message")?;
        assert_eq!(
            decode_encrypted_message(&skey, &signed_text).with_context(|| "Decrypting message")?,
            plaintext
        );
        Ok(())
    }
}
