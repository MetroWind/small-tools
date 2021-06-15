use log::error as log_error;
use futures::StreamExt;
use telegram_bot as bot;
use bot::types::requests::SendMessage;
use bot::types::Message;
use tokio;
use async_trait::async_trait;

use crate::error::Error;
use crate::config;

pub async fn sendMessage(api: &bot::Api, msg: String, to: bot::UserId)
    -> Result<bot::types::Message, Error>
{
    let post = api.send(
        SendMessage::new(to, msg).parse_mode(bot::types::ParseMode::Markdown)
            .disable_preview())
        .await.map_err(|_| error!(RuntimeError, "Failed to send message"))?;
    if let bot::types::MessageOrChannelPost::Message(msg) = post
    {
        Ok(msg)
    }
    else
    {
        Err(error!(RuntimeError, "Shit happened."))
    }
}

#[async_trait]
pub trait MessageHandler
{
    fn new(conf: &config::ConfigParams) -> Self;
    fn config(&self) -> &config::ConfigParams;
    async fn onMessage(&self, api: &bot::Api, msg: Message) -> Result<(), Error>;

    fn listen(&self) where Self: std::marker::Sync
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let api = bot::Api::new(&self.config().token);
        let mut stream = api.stream();
        while let Some(update) = rt.block_on(async{ stream.next().await })
        {
            let update = match update
            {
                Err(e) => {log_error!("{}", e); continue;},
                Ok(u) => u,
            };

            match update.kind
            {
                bot::types::UpdateKind::Message(message) =>
                {
                    if let Err(e) = rt.block_on(async{ self.onMessage(&api, message).await})
                    {
                        log_error!("{}", e);
                    }
                },
                bot::types::UpdateKind::ChannelPost(_) => (),
                _ => (),
            }
        }
    }
}

#[derive(Clone)]
pub struct GetUIDHandler
{
    config: config::ConfigParams,
}

#[async_trait]
impl MessageHandler for GetUIDHandler
{
    fn new(conf: &config::ConfigParams) -> Self
    {
        Self{ config: conf.clone() }
    }

    fn config(&self) -> &config::ConfigParams
    {
        &self.config
    }

    async fn onMessage(&self, api: &bot::Api, msg: Message) -> Result<(), Error>
    {
        let uid = msg.from.id;
        sendMessage(api, format!("Your UID is {}.", uid), uid).await?;
        Ok(())
    }
}
