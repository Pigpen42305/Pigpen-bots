use clap::Parser;
use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

use crate::{PREFIX, message_utils, parser::BotParser};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        match (&msg.content).trim().strip_prefix(PREFIX).map(|x| x.trim()) {
            Some(x) => msg.content = String::from(x),
            None => return,
        }

        let args = match shlex::split(&msg.content) {
            None => {
                _ = message_utils::message(&ctx, &msg, "Syntax error!").await;
                return;
            }
            Some(x) => x,
        };

        let call = match BotParser::try_parse_from(args.into_iter()) {
            Ok(call) => call,
            Err(err) => {
                _ = message_utils::message_split(
                    &ctx,
                    &msg,
                    err.to_string(),
                    Some(("```text\n", "\n```")),
                )
                .await;
                return;
            }
        };

        let (block, output) = call.execute().expect("A fatal error has occurred!");

        _ = match block {
            Some(lang) => {
                message_utils::message_split(
                    &ctx,
                    &msg,
                    output,
                    Some((format!("```{lang}\n"), "\n```")),
                )
                .await
            }
            None => message_utils::message_split::<!, !>(&ctx, &msg, output, None).await,
        };
    }
}
