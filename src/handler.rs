use clap::Parser;
use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

use crate::{
    PREFIX,
    commands::DiscordOutput,
    message_utils::{self, MessageOptions},
    parser::BotParser,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        match (&msg.content).trim().strip_prefix(PREFIX).map(|x| x.trim()) {
            Some(x) => msg.content = String::from(x),
            None => return,
        }

        let mut options = MessageOptions::default();

        let args = match shlex::split(&msg.content) {
            None => {
                _ = message_utils::silent_message(&ctx, &msg, "Syntax error!", options).await;
                return;
            }
            Some(x) => x,
        };

        let call = match BotParser::try_parse_from(args.into_iter()) {
            Ok(call) => call,
            Err(err) => {
                _ = message_utils::silent_message_in_chunks(
                    &ctx,
                    &msg,
                    err.to_string(),
                    Some(("```text\n", "\n```")),
                    options,
                )
                .await;
                return;
            }
        };

        let DiscordOutput {
            block,
            content,
            suppress_embeds,
        } = call.execute().expect("A fatal error has occurred!");

        options.suppress_embeds = suppress_embeds;

        _ = match block {
            Some(lang) => {
                message_utils::silent_message_in_chunks(
                    &ctx,
                    &msg,
                    content,
                    Some((format!("```{lang}\n"), "\n```")),
                    options,
                )
                .await
            }
            None => {
                message_utils::silent_message_in_chunks::<!, !>(&ctx, &msg, content, None, options)
                    .await
            }
        };
    }
}
