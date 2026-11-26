use serenity::{
    Error as SerenityError,
    all::{Context, CreateMessage, Message, MessageFlags, ModelError},
};

use crate::MAX_MESSAGE_LEN;

fn message_chunks_helper(input: impl Into<String>, len: usize) -> Vec<String> {
    let string: String = input.into();
    let mut parts = Vec::new();
    let mut iter = string.chars();
    loop {
        let mut string_val = String::with_capacity(len);
        for _ in 0..len {
            match iter.next() {
                Some(c) => string_val.push(c),
                None => break,
            }
        }
        parts.push(string_val);
        return parts;
    }
}

fn message_chunks(input: impl Into<String>) -> Vec<String> {
    message_chunks_helper(input, MAX_MESSAGE_LEN)
}

fn wrapped_message_chunks(
    input: impl Into<String>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
) -> Vec<String> {
    let pre: &str = prefix.as_ref();
    let suf: &str = suffix.as_ref();
    message_chunks_helper(
        input,
        MAX_MESSAGE_LEN.strict_sub(pre.len()).strict_sub(suf.len()),
    )
    .into_iter()
    .map(|x| format!("{pre}{x}{suf}"))
    .collect()
}

async fn message_helper(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
    silent: bool,
) -> Result<(), SerenityError> {
    let flags = if silent {
        MessageFlags::SUPPRESS_NOTIFICATIONS
    } else {
        MessageFlags::empty()
    };
    match msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().content(value).flags(flags))
        .await
    {
        Ok(_) => Ok(()),
        Err(err @ SerenityError::Model(ModelError::MessageTooLong(code))) => {
            if let Err(why) = msg
                .channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content(format!("Output too long: {code}"))
                        .flags(flags),
                )
                .await
            {
                eprintln!("Error: {why:#?}");
                Err(err)
            } else {
                Ok(())
            }
        }
        Err(err) => {
            if let Err(why) = msg
                .channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content(format!("Error: {err:#?}"))
                        .flags(flags),
                )
                .await
            {
                eprintln!("Error: {why:#?}");
            }
            Err(err)
        }
    }
}

pub async fn silent_message(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    let value = value.into();
    assert!(value.len() <= MAX_MESSAGE_LEN, "MAX LENGTH EXCEEDED!");
    message_helper(ctx, msg, value, true).await
}

#[expect(dead_code, reason = "We may want the option later...")]
pub async fn message(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    let value = value.into();
    assert!(value.len() <= MAX_MESSAGE_LEN, "MAX LENGTH EXCEEDED!");
    message_helper(ctx, msg, value, false).await
}

#[expect(dead_code, reason = "We may want the option later...")]
pub async fn message_in_chunks<A: AsRef<str>, B: AsRef<str>>(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
    wrapper: Option<(A, B)>,
) -> Result<(), SerenityError> {
    let parts = match wrapper {
        Some((prefix, suffix)) => wrapped_message_chunks(value, prefix, suffix),
        None => message_chunks(value),
    };
    for (i, string) in parts.iter().enumerate() {
        debug_assert!(string.len() <= MAX_MESSAGE_LEN);
        message_helper(ctx, msg, string, i != 0).await?;
    }
    Ok(())
}

pub async fn silent_message_in_chunks<A: AsRef<str>, B: AsRef<str>>(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
    wrapper: Option<(A, B)>,
) -> Result<(), SerenityError> {
    let parts = match wrapper {
        Some((prefix, suffix)) => wrapped_message_chunks(value, prefix, suffix),
        None => message_chunks(value),
    };
    for string in parts.iter() {
        debug_assert!(string.len() <= MAX_MESSAGE_LEN);
        silent_message(ctx, msg, string).await?;
    }
    Ok(())
}
