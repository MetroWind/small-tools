#![allow(non_snake_case)]

#[macro_use]
mod error;
mod auth;

fn formatBytes(bytes: &[u8]) -> String
{
    bytes.iter().map(|b| format!("{:x}", b)).collect()
}

fn main()
{
    println!("{}", formatBytes(&auth::Authenticator::new().randBytes(10)));
}
