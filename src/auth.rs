// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// http://www.wtfpl.net/ for more details.

use std::iter::FromIterator;

use rand::RngCore;
use rand::SeedableRng;
use num_bigint::BigUint;
use num_traits::Pow;
use num_traits::identities::Zero;
use num_traits::cast::ToPrimitive;
use num_integer::Integer;
use reqwest::blocking as requests;

use crate::error;
use crate::error::Error;

pub struct Authenticator
{
    // prepare a non-deterministic random number generator:
    rng: rand::rngs::StdRng,
    rsa_key: Vec<u8>,

    key: Vec<u8>,
    serial: String,
}

impl Authenticator
{
    const REGION: &'static str = "US"; // Must be 2 bytes.
    const MODEL: &'static str = "Motorola RAZR v3";

    pub fn key(&self) -> &[u8]
    {
        &self.key
    }

    pub fn serial(&self) -> &str
    {
        &self.serial
    }

    pub fn new() -> Self
    {
        Self {
            rng: rand::rngs::StdRng::from_entropy(),
            rsa_key: Vec::new(),
            key: Vec::new(),
            serial: String::new(),
        }
    }

    fn randBytes(&mut self, size: usize) -> Vec<u8>
    {
        let mut result: Vec<u8> = vec![0;size];
        self.rng.fill_bytes(&mut result);
        return result;
    }

    fn requestDataClear(&mut self) -> Vec<u8>
    {
        self.rsa_key = self.randBytes(37);
        let mut base: Vec<u8> = vec![1,];
        base.extend(self.rsa_key.iter());
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
        let data_num = BigUint::from_bytes_be(data);
        let modulus = BigUint::parse_bytes(b"955e4bd989f3917d2f15544a7e0504eb9d7bb66b6f8a2fe470e453c779200e5e3ad2e43a02d06c4adbd8d328f1a426b83658e88bfd949b2af4eaf30054673a1419a250fa4cc1278d12855b5b25818d162c6e6ee2ab4a350d401d78f6ddb99711e72626b48bd8b5b0b7f3acf9ea3c9e0005fee59e19136cdb7c83f2ab8b0a2a99", 16)
            .ok_or_else(|| error!(RuntimeError, "Failed to parse modulo"))?;
        let exp = BigUint::from(0x101 as u32);

        let mut n = data_num.pow(exp) % modulus;
        let mut result: Vec<u8> = Vec::new();
        let two_five_six = BigUint::from(256 as u32);
        while !n.is_zero()
        {
            let (new_n, m) = n.div_rem(&two_five_six);
            result.push(m.to_u8().unwrap());
            n = new_n;
        }
        result.reverse();
        Ok(result)
    }

    pub fn makeRequest(&mut self) -> Result<Vec<u8>, Error>
    {
        let client = requests::Client::new();
        let req_data_clear = self.requestDataClear();
        let req_data = self.encrypt(&req_data_clear)?;
        let res = client.post("http://mobile-service.blizzard.com/enrollment/enroll.htm")
            .body(req_data)
            .header("Content-Type", "application/octet-stream")
            .send().map_err(|_| error!(RuntimeError, "Failed to post"))?;
        Ok(Vec::from_iter(res.bytes().map_err(
            |_| error!(RuntimeError, "Failed to get bytes from response"))?))
    }

    pub fn decrypt(&self, res: &[u8]) -> Vec<u8>
    {
        let data = &res[8..];
        let mut result = Vec::new();
        for i in 0..37
        {
            result.push(data[i] ^ self.rsa_key[i]);
        }
        result
    }

    pub fn request(&mut self) -> Result<(), Error>
    {
        let res = self.makeRequest()?;
        let res = self.decrypt(&res);
        self.key.resize(20, 0);
        self.key.copy_from_slice(&res[..20]);
        self.serial = std::str::from_utf8(&res[20..]).map_err(
            |_| error!(RuntimeError, "Failed to decode UTF-8"))?.to_owned();
        Ok(())
    }
}
