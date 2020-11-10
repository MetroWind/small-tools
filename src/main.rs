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
    let image = code.render::<Luma<u8>>().build();
    image.save(filename).map_err(
        |_| error!(RuntimeError, "Failed to save image"))?;
    Ok(())
}

fn main()
{
    let opts = clap::App::new("Battle.net authenticator request")
        .version("0.1")
        .author("MetroWind")
        .about("Does awesome things")
        .arg(clap::Arg::with_name("USER")
             .required(true)
             .help("User login name of the account"))
        .arg(clap::Arg::with_name("qrcode")
             .long("qr-code")
             .value_name("FILE")
             .takes_value(true)
             .help("Generate a QR code at FILE."))
        .get_matches();

    let mut bnauth = auth::Authenticator::new();
    bnauth.request().unwrap();
    println!("Serial number: {}", bnauth.serial());

    let user = opts.value_of("USER").unwrap();
    if opts.is_present("qrcode")
    {
        key2Qr(bnauth.key(), user, opts.value_of("qrcode").unwrap()).unwrap();
    }
    else
    {
        println!("OTP URL: {}", key2Url(bnauth.key(), user))
    }
}
