use rand::RngCore;
use rand::SeedableRng;
use rsa::BigUint;
use rsa::PublicKey;

use crate::error;
use crate::error::Error;

pub struct Authenticator
{
    // prepare a non-deterministic random number generator:
    rng: rand::rngs::StdRng,
}

impl Authenticator
{
    const REGION: &'static str = "US"; // Must be 2 bytes.
    const MODEL: &'static str = "Motorola RAZR v3";

    pub fn new() -> Self
    {
        Self { rng: rand::rngs::StdRng::from_entropy() }
    }

    pub fn randBytes(&mut self, size: usize) -> Vec<u8>
    {
        let mut result: Vec<u8> = vec![0;size];
        self.rng.fill_bytes(&mut result);
        return result;
    }

    fn requestDataClear(&mut self) -> Vec<u8>
    {
        let otp = self.randBytes(37);
        let mut base: Vec<u8> = vec![1,];
        base.extend(otp);
        base.extend(Self::REGION.to_owned().bytes());
        base.extend(Self::MODEL.to_owned().bytes().take(16));
        for _ in 0..(16 - Self::MODEL.len())
        {
            base.push(0);
        }

        base
    }

    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, Error>
    {
        let modulus = BigUint::parse_bytes(b"955e4bd989f3917d2f15544a7e0504eb9d7bb66b6f8a2fe470e453c779200e5e3ad2e43a02d06c4adbd8d328f1a426b83658e88bfd949b2af4eaf30054673a1419a250fa4cc1278d12855b5b25818d162c6e6ee2ab4a350d401d78f6ddb99711e72626b48bd8b5b0b7f3acf9ea3c9e0005fee59e19136cdb7c83f2ab8b0a2a99", 16)
            .ok_or_else(|| error!(RuntimeError, "Failed to parse modulo"))?;
        let exp = BigUint::from(0x101 as u32);

        let pubkey = rsa::RSAPublicKey::new(modulus, exp).map_err(
            |_| error!(RuntimeError, "Failed to create public key"))?;
        pubkey.encrypt(&mut self.rng,
                       rsa::padding::PaddingScheme::PKCS1v15Encrypt,
                       data)
            .map_err(|_| error!(RuntimeError, "Failed to encrypt"))
    }
}
