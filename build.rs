// A build script used in GitHub flow.

#![allow(non_snake_case)]

use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::process::Command;

static SKIPS: &'static [&'static str] = &[".github", ".git", "bin"];

macro_rules! error
{
    ( $err_type:ident, $msg:literal ) =>
    {
        {
            Error::$err_type(String::from($msg))
        }
    };
    ( $err_type:ident, $msg:literal $(, $x:expr)+) =>
    {
        {
            Error::$err_type(format!($msg $(, $x)+))
        }
    };
}

// Construct a RuntimeError
#[macro_export]
macro_rules! rterr
{
    ($msg:literal $(, $x:expr)*) =>
    {
        error!(RuntimeError, $msg $(, $x)*)
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error
{
    RuntimeError(String),
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl StdError for Error
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {None}
}

// fn buildInDir(dir: &str) -> Result<(), Error>
// {
//     let status = Command::new("cargo")
//         .args(&["build", "--manifest-path", &format!("{}/Cargo.toml", dir)])
//         .status().map_err(|e| rterr!("Failed to run cargo build: {}", e))?;
//     if status.success()
//     {
//         Ok(())
//     }
//     else
//     {
//         Err(rterr!("Cargo build failed with code {}", status))
//     }
// }

fn installBinary(dir: &str, prefix: &str) -> Result<(), Error>
{
    let status = Command::new("cargo")
        .args(&["install", "--path", dir, "--root", prefix, "--force"])
        .status().map_err(|e| rterr!("Failed to run cargo install: {}", e))?;
    if status.success()
    {
        Ok(())
    }
    else
    {
        Err(rterr!("Cargo install failed with code {}", status))
    }
}

fn packAll() -> Result<(), Error>
{
    for entry in fs::read_dir("bin")
        .map_err(|_| rterr!("Failed to list directory"))?
    {
        let entry = entry.map_err(
            |_| rterr!("Failed to get bin directory entry"))?;
        let path = entry.path();
        let path_str = path.to_str().ok_or_else(
            || rterr!("Failed to encode bin path"))?;

        println!("Stripping {}...", path_str);
        let status = Command::new("strip")
            .arg(path_str)
            .status().map_err(|e| rterr!("Failed to run strip: {}", e))?;
        if !status.success()
        {
            println!("Error: Failed to stipe {} with code {}", path_str, status);
            continue;
        }

        println!("Compressing {}...", path_str);
        let status = Command::new("xz")
            .arg(path_str)
            .status().map_err(|e| rterr!("Failed to run xz: {}", e))?;
        if !status.success()
        {
            println!("Error: Failed to compress {} with code {}", path_str, status);
            continue;
        }

        // Rename binary
        if fs::rename(format!("{}.xz", path_str),
                      format!("{}-{}-{}.xz", path_str, std::env::consts::OS,
                              std::env::consts::ARCH)).is_err()
        {
            println!("Error: Failed to rename {}.xz.", path_str);
        }
    }
    Ok(())
}

fn build() -> Result<(), Error>
{
    for entry in fs::read_dir(".")
        .map_err(|_| rterr!("Failed to list directory"))?
    {
        let entry = entry.map_err(|_| rterr!("Failed to get directory entry"))?;
        let path = entry.path();
        if !path.is_dir() { continue; }

        let dir = path.file_name().ok_or_else(|| rterr!("Invalid path"))?
            .to_str().ok_or_else(|| rterr!("Failed to encode path"))?;
        if SKIPS.iter().position(|e| e == &dir).is_some()
        {
            continue;
        }
        let path_str = path.to_str().ok_or_else(
            || rterr!("Failed to encoding directory path"))?;
        println!("Building {}...", path_str);
        let result = installBinary(path_str, ".");
        if let Err(err) = result
        {
            println!("{}", err);
        }
    }
    Ok(())
}

fn main() -> Result<(), Error>
{
    build()?;
    packAll()
}
