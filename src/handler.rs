// ChatGPT was used to implement logging...
//
// Further review encouraged!
//
// TODO: Further investigate and understand the new implementation.
//

use std::{panic::AssertUnwindSafe, time::Instant};

use clap::Parser;
use serenity::{
    FutureExt,
    all::{Context, EventHandler, Message},
    async_trait,
};
use tracing::{Instrument, error, info, warn};

use crate::{
    PREFIX,
    commands::DiscordOutput,
    logging::command_span,
    message_utils::{self, MessageOptions},
    parser::BotParser,
};

#[derive(Debug)]
pub struct Handler;

impl Handler {
    async fn handle_command(&self, ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
        info!(
            input = ?msg.content,
            "Command recieved",
        );
        let mut options = MessageOptions::default();

        let args = match shlex::split(&msg.content) {
            Some(args) => args,
            None => {
                warn!("Invalid command syntax");

                return message_utils::silent_message(ctx, msg, "Syntax error!", options).await;
            }
        };

        let call = match BotParser::try_parse_from(args) {
            Ok(call) => call,
            Err(err) => {
                if err.use_stderr() {
                    warn!(
                        kind = ?err.kind(),
                        error = %err,
                        "Command parsing failed",
                    );
                } else {
                    info!(
                        kind = ?err.kind(),
                        "Parser information requested",
                    );
                }

                return message_utils::silent_message_in_chunks(
                    ctx,
                    msg,
                    err.to_string(),
                    Some(("```text\n", "\n```")),
                    options,
                )
                .await;
            }
        };
        info!(
            call = ?call,
            "Command parsed",
        );

        let DiscordOutput {
            block,
            content,
            suppress_embeds,
        } = call.execute().expect("A fatal error has occurred!");

        options.suppress_embeds = suppress_embeds;

        match block {
            Some(lang) => {
                message_utils::silent_message_in_chunks(
                    ctx,
                    msg,
                    content,
                    Some((format!("```{lang}\n"), "\n```")),
                    options,
                )
                .await
            }

            None => {
                message_utils::silent_message_in_chunks::<!, !>(ctx, msg, content, None, options)
                    .await
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, mut msg: Message) {
        if msg.author.bot {
            return;
        }

        let Some(input) = msg.content.trim().strip_prefix(PREFIX) else {
            return;
        };

        msg.content = input.trim().to_owned();

        let span = command_span(&ctx, &msg);

        async {
            let start = Instant::now();

            let result = AssertUnwindSafe(self.handle_command(&ctx, &msg))
                .catch_unwind()
                .await;

            let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

            match result {
                Ok(Ok(())) => {
                    info!(elapsed_ms, "Command handled",);
                }
                Ok(Err(err)) => {
                    error!(
                        error = %err,
                        elapsed_ms,
                        "Command handling failed",
                    );
                }
                Err(_) => {
                    error!(elapsed_ms, "Command panicked",);
                    if let Err(err) = message_utils::silent_message(
                        &ctx,
                        &msg,
                        "An internal error occurred.",
                        MessageOptions::default(),
                    )
                    .await
                    {
                        error!(
                            error = %err,
                            "Failed to send panic notification",
                        );
                    }
                }
            }
        }
        .instrument(span)
        .await;
    }
}
