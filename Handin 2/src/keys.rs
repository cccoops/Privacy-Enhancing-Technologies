use base64::{engine::general_purpose, Engine as _};
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Struct to hold public and private key pair
#[derive(Debug)]
pub struct KeyPair {
    pub private_key: Scalar,
    pub public_key: RistrettoPoint,
}

impl KeyPair {
    /// Generate a Schnorr-style key pair
    pub fn generate() -> KeyPair {
        let mut csprng = OsRng;
        let private_key = Scalar::random(&mut csprng);
        let public_key = private_key * &RISTRETTO_BASEPOINT_POINT;

        KeyPair {
            private_key,
            public_key,
        }
    }

    /// Read and base64-decode a file
    fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, io::Error> {
        let mut encoded = String::new();
        File::open(path)?.read_to_string(&mut encoded)?;

        general_purpose::STANDARD
            .decode(encoded.trim())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid base64"))
    }

    pub fn write_sk_to_file(&self, sk_filepath: &str) -> Result<(), io::Error> {
        let encoded = general_purpose::STANDARD.encode(&self.private_key.to_bytes());
        let mut file = File::create(sk_filepath)?;
        file.write_all(encoded.as_bytes())?;
        Ok(())
    }

    pub fn write_pk_to_file(&self, pk_filepath: &str) -> Result<(), io::Error> {
        let encoded = general_purpose::STANDARD.encode(&self.public_key.compress().to_bytes());
        let mut file = File::create(pk_filepath)?;
        file.write_all(encoded.as_bytes())?;
        Ok(())
    }

    /// Load keypair from secret key file
    pub fn from_file(sk_filepath: &str) -> Result<KeyPair, io::Error> {
        let sk_bytes = Self::read_file(sk_filepath)?;

        let sk_array: [u8; 32] = sk_bytes
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid scalar length"))?;

        let private_key = Scalar::from_bytes_mod_order(sk_array);
        let public_key = private_key * &RISTRETTO_BASEPOINT_POINT;

        Ok(KeyPair {
            private_key,
            public_key,
        })
    }

    pub fn pk_from_file(pk_filepath: &str) -> Result<RistrettoPoint, io::Error> {
        let pk_bytes = Self::read_file(pk_filepath)?;

        let pk_array: [u8; 32] = pk_bytes
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid public key length"))?;

        let compressed = CompressedRistretto(pk_array);
        compressed.decompress().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid compressed Ristretto point",
            )
        })
    }
}

// Unit tests for keys module
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_keypair() {
        let keypair = KeyPair::generate();
        assert!(
            keypair.public_key != RistrettoPoint::default(),
            "Public key should not be default"
        );
        assert!(
            keypair.private_key != Scalar::default(),
            "Private key should not be default"
        );
        assert!(
            keypair.private_key * &RISTRETTO_BASEPOINT_POINT == keypair.public_key,
            "Public key should be g^private_key"
        )
    }

    #[test]
    fn test_write_and_read_keypair() {
        let keypair = KeyPair::generate();
        let pk_filepath = "pk_test.txt";
        let sk_filepath = "sk_test.txt";

        // Write the keypair to a file
        keypair
            .write_sk_to_file(&sk_filepath)
            .expect("Failed to write sk to file");
        keypair
            .write_pk_to_file(&pk_filepath)
            .expect("Failed to write pk to file");

        // Read the keypair back from the file
        let read_keypair =
            KeyPair::from_file(&sk_filepath).expect("Failed to read keypair from file");

        let read_pk = KeyPair::pk_from_file(&pk_filepath).expect("Failed to read pk from file");

        // Check if the written and read key pairs are equal
        assert_eq!(
            keypair.private_key, read_keypair.private_key,
            "Private keys should match"
        );
        assert_eq!(
            keypair.public_key, read_keypair.public_key,
            "Public keys should match"
        );
        assert_eq!(keypair.public_key, read_pk, "Public keys should match");

        // Clean up the test file
        fs::remove_file(&sk_filepath).expect("Failed to remove sk test file");
        fs::remove_file(&pk_filepath).expect("Failed to remove pk test file");
    }
}
