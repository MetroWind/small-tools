#![allow(non_snake_case)]

use std::io::{self, Read};
use std::process::exit;

use clap;
use log::debug;
use tokio;
use log::error as log_error;
use telegram_bot as bot;
use stderrlog;

#[macro_use]
mod error;
mod telegram;
mod config;

use error::Error;

use crate::telegram::MessageHandler;

fn readConfig() -> Result<config::ConfigParams, Error>
{
    let conf_file = "telegram-bot-control.toml";
    debug!("Reading config from {}...", conf_file);
    config::ConfigParams::fromFile(conf_file)
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
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(telegram::sendMessage(&api, content, bot::types::UserId::new(uid)))?;
        Ok(())
    }
    else
    {
        Err(error!(RuntimeError, "User ID not found in config"))
    }
}

fn main()
{
    stderrlog::new().init().unwrap();
    let opts = clap::App::new("Telegram bot control")
        .version("0.1")
        .author("MetroWind <chris.corsair@gmail.com>")
        .about("Control a telegram bot.")
        .subcommand(clap::App::new("get-uid")
                    .about("Get user ID"))
        .get_matches();

    let conf = match readConfig()
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
