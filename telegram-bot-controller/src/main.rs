#![allow(non_snake_case)]

use std::io::{self, Read};
use std::process::exit;
use std::str::FromStr;
use std::path::PathBuf;
use std::env;

use log::debug;
use tokio;
use log::error as log_error;
use telegram_bot as bot;
use stderrlog;
use clap;

#[macro_use]
mod error;
mod telegram;
mod config;

use error::Error;

use crate::telegram::MessageHandler;

const CONF_FILE: &str = "telegram-bot-control.toml";

fn readConfig(path: &std::path::Path) -> Result<config::ConfigParams, Error>
{
    debug!("Reading config from {}...", path.to_string_lossy());
    config::ConfigParams::fromFile(path)
}

fn sendStdin(conf: &config::ConfigParams) -> Result<(), Error>
{
    if let Some(uid) = conf.notify_to
    {
        let mut content = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content).map_err(
            |_| error!(RuntimeError, "Failed to read stdin"))?;

        let api = bot::Api::new(&conf.token);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(telegram::sendMessage(&api, content, bot::types::UserId::new(uid)))?;
        Ok(())
    }
    else
    {
        Err(error!(RuntimeError, "User ID not found in config"))
    }
}

fn findConfigFile() -> Option<PathBuf>
{
    if let Ok(dir) = env::var("HOME")
    {
        if !dir.is_empty()
        {
            let mut path = PathBuf::from(dir);
            path.push(".config");
            path.push(CONF_FILE);
            if path.exists()
            {
                return Some(path);
            }
        }
    }

    if let Ok(mut path) = PathBuf::from_str("/etc")
    {
        path.push(CONF_FILE);
        if path.exists()
        {
            return Some(path);
        }
    }
    None
}


fn main()
{
    stderrlog::new().init().unwrap();
    let opts = clap::Command::new("Telegram bot control")
        .version("0.1.1")
        .author("MetroWind <chris.corsair@gmail.com>")
        .about("Control a telegram bot.")
        .arg(clap::Arg::new("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .help("Location of config file. Default: ~/.config/telegram-bot-control.toml or /etc/telegram-bot-control.toml"))
        .subcommand(clap::Command::new("get-uid")
                    .about("Get user ID"))
        .get_matches();

    let conf_file = if let Some(path) = findConfigFile()
    {
        path
    }
    else if let Some(config) = opts.get_one::<String>("config")
    {
        if let Ok(path) = PathBuf::from_str(config)
        {
            path
        }
        else
        {
            log_error!("Invalid config path: {}", config);
            exit(3);
        }
    }
    else
    {
        log_error!("Cannot find config file");
        exit(3);
    };

    let conf = match readConfig(&conf_file)
    {
        Ok(conf) => conf,
        Err(e) => { log_error!("{}", e); exit(2); },
    };
    match opts.subcommand_name()
    {
        Some("get-uid") =>
        {
            println!("Sent me a message, and I will tell you your UID.");
            let handler = telegram::GetUIDHandler::new(&conf);
            handler.listen();
        },

        None =>
        {
            if let Err(e) = sendStdin(&conf)
            {
                log_error!("{}", e);
                exit(1);
            }
        },

        Some(a) =>
        {
            log_error!("Unknown command: {}", a);
            exit(-1);
        },
    }
}
