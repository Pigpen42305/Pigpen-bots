// ChatGPT was used for this file...
//
// Further review encouraged!
//

use std::{env::home_dir, fs::create_dir_all, io, path::PathBuf};

use serenity::all::{Context, Message};
use tracing::{Span, info_span};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::fmt;

fn logging_directory() -> io::Result<PathBuf> {
    let mut home = match home_dir() {
        Some(x) => x,
        None => panic!("Could not find the home directory..."),
    };
    home.push(".logs");
    home.push("discord-bot");
    create_dir_all(&home)?;
    Ok(home)
}

pub fn command_span(ctx: &Context, msg: &Message) -> Span {
    let mut guild_name = None;
    let mut channel_name = None;

    let mut guild_nickname = msg.member.as_ref().and_then(|member| member.nick.clone());

    if let Some(guild_id) = msg.guild_id {
        if let Some(guild) = ctx.cache.guild(guild_id) {
            guild_name = Some(guild.name.clone());

            channel_name = guild
                .channels
                .get(&msg.channel_id)
                .or_else(|| {
                    guild
                        .threads
                        .iter()
                        .find(|thread| thread.id == msg.channel_id)
                })
                .map(|channel| channel.name.clone());

            if guild_nickname.is_none() {
                guild_nickname = guild
                    .members
                    .get(&msg.author.id)
                    .and_then(|member| member.nick.clone());
            }
        }
    }

    let display_name = guild_nickname
        .as_deref()
        .or(msg.author.global_name.as_deref())
        .unwrap_or(&msg.author.name);

    info_span!(
        "command",

        guild_id = ?msg.guild_id,
        guild_name = ?guild_name,

        user_id = %msg.author.id,
        username = %msg.author.name,
        global_name = ?msg.author.global_name,
        guild_nickname = ?guild_nickname,
        display_name = %display_name,

        channel_id = %msg.channel_id,
        channel_name = ?channel_name,

        message_id = %msg.id,
        message_timestamp = %msg.timestamp,
    )
}

pub fn setup_logging() -> io::Result<WorkerGuard> {
    let dir = logging_directory()?;

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY)
        .filename_prefix("bot")
        .filename_suffix("log")
        .max_log_files(24)
        .build(dir)
        .map_err(io::Error::other)?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    fmt().with_writer(non_blocking).with_ansi(false).init();

    Ok(guard)
}
