use serenity::{
    Error as SerenityError,
    all::{Context, Message, ModelError},
};

use crate::MAX_MESSAGE_LEN;

const SILENT: &str = "@silent ";
const MAXLEN_SILENT: usize = MAX_MESSAGE_LEN - SILENT.len();

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

fn silent_message_chunks(input: impl Into<String>) -> Vec<String> {
    message_chunks_helper(input, MAXLEN_SILENT)
}

fn message_chunks(input: impl Into<String>) -> Vec<String> {
    message_chunks_helper(input, MAX_MESSAGE_LEN)
}

fn wrapped_message_chunks_helper(
    input: impl Into<String>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
    len: usize,
) -> Vec<String> {
    let pre: &str = prefix.as_ref();
    let suf: &str = suffix.as_ref();
    message_chunks_helper(input, len.strict_sub(pre.len()).strict_sub(suf.len()))
        .into_iter()
        .map(|x| format!("{pre}{x}{suf}"))
        .collect()
}

fn wrapped_silent_message_chunks(
    input: impl Into<String>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
) -> Vec<String> {
    wrapped_message_chunks_helper(input, prefix, suffix, MAXLEN_SILENT)
}

fn wrapped_message_chunks(
    input: impl Into<String>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
) -> Vec<String> {
    let input = input.into();
    let pre: &str = prefix.as_ref();
    let suf: &str = suffix.as_ref();
    let first = MAX_MESSAGE_LEN.strict_sub(pre.len()).strict_sub(suf.len());
    if input.len() <= first {
        vec![format!("{pre}{input}{suf}")]
    } else {
        let mut iter = input.chars();
        let mut parts = Vec::new();
        let first_part = parts.push_mut(String::with_capacity(first));
        first_part.push_str(pre);
        for _ in 0..first {
            let Some(ch) = iter.next() else {
                unreachable!()
            };
            first_part.push(ch);
        }
        first_part.push_str(suf);
        let input: String = iter.collect();
        wrapped_silent_message_chunks(input, prefix, suffix)
            .into_iter()
            .fold(parts, |mut p, x| {
                p.push(x);
                p
            })
    }
}

pub async fn silent_message(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    let value = format!("{SILENT}{}", value.into());
    assert!(value.len() <= MAX_MESSAGE_LEN, "MAX LENGTH EXCEEDED!");
    message_helper(ctx, msg, value).await
}

pub async fn message(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    let value = value.into();
    assert!(value.len() <= MAX_MESSAGE_LEN, "MAX LENGTH EXCEEDED!");
    message_helper(ctx, msg, value).await
}

async fn message_helper(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    match msg.channel_id.say(&ctx.http, value).await {
        Ok(_) => Ok(()),
        Err(err @ SerenityError::Model(ModelError::MessageTooLong(code))) => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("Output too long: {code}"))
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
                .say(&ctx.http, format!("Error: {err:#?}"))
                .await
            {
                eprintln!("Error: {why:#?}");
            }
            Err(err)
        }
    }
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
        if i == 0 {
            debug_assert!(string.len() <= MAX_MESSAGE_LEN);
            message(ctx, msg, string).await?;
        } else {
            debug_assert!(string.len() <= MAXLEN_SILENT);
            silent_message(ctx, msg, string).await?;
        }
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
        Some((prefix, suffix)) => wrapped_silent_message_chunks(value, prefix, suffix),
        None => silent_message_chunks(value),
    };
    for string in parts.iter() {
        debug_assert!(string.len() <= MAXLEN_SILENT);
        silent_message(ctx, msg, string).await?;
    }
    Ok(())
}
