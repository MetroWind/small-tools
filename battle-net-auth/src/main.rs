// This program is free software. It comes without any warranty, to
// the extent permitted by applicable law. You can redistribute it
// and/or modify it under the terms of the Do What The Fuck You Want
// To Public License, Version 2, as published by Sam Hocevar. See
// http://www.wtfpl.net/ for more details.

#![allow(non_snake_case)]

use data_encoding::BASE32;
use qrcode::QrCode;
use image::Luma;

#[macro_use]
mod error;
mod auth;

use crate::error::Error;

fn formatBytes(bytes: &[u8]) -> String
{
    bytes.iter().map(|b| format!("{:x}", b)).collect()
}

fn key2Url(key: &[u8], user: &str) -> String
{
    format!("otpauth://totp/{}?secret={}&issuer=Blizzard",
            user, BASE32.encode(key))
}

fn key2Qr(key: &[u8], user: &str, filename: &str) -> Result<(), Error>
{
    let code = QrCode::new(key2Url(key, user).as_bytes()).map_err(
        |_| error!(RuntimeError, "Failed to QR encode"))?;
    let img = code.render::<Luma<u8>>().build();
    img.save(filename).map_err(
        |_| error!(RuntimeError, "Failed to save image"))?;
    Ok(())
}

fn main()
{
    let opts = clap::Command::new("Battle.net authenticator request")
        .version("0.1")
        .author("MetroWind")
        .about("Does awesome things")
        .arg(clap::Arg::new("USER")
             .required(true)
             .help("User login name of the account"))
        .arg(clap::Arg::new("qrcode")
             .long("qr-code")
             .value_name("FILE")
             .help("Generate a QR code to FILE."))
        .get_matches();

    let mut bnauth = auth::Authenticator::new();
    bnauth.request().unwrap();
    println!("Serial number: {}", bnauth.serial());

    let user = opts.get_one::<String>("USER").unwrap();
    if let Some(q) = opts.get_one::<String>("qrcode")
    {
        key2Qr(bnauth.key(), user, q).unwrap();
    }
    else
    {
        println!("OTP URL: {}", key2Url(bnauth.key(), user))
    }
}
