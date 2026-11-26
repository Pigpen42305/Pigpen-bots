use serenity::{
    Error as SerenityError,
    all::{Context, Message, ModelError},
};

use crate::MAX_MESSAGE_LEN;

const SILENT: &str = "@silent ";
const MAXLEN_SILENT: usize = MAX_MESSAGE_LEN - SILENT.len();

fn split_message_helper(input: impl Into<String>, len: usize) -> Vec<String> {
    assert!(len > 0);
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

pub fn split_message(input: impl Into<String>) -> Vec<String> {
    split_message_helper(input, MAXLEN_SILENT)
        .into_iter()
        .map(|x| format!("{SILENT}{x}"))
        .collect()
}

pub fn split_message_wrapped(
    input: impl Into<String>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
) -> Vec<String> {
    let pre: &str = prefix.as_ref();
    let suf: &str = suffix.as_ref();
    split_message_helper(
        input,
        MAXLEN_SILENT.strict_sub(pre.len()).strict_sub(suf.len()),
    )
    .into_iter()
    .map(|x| format!("{SILENT}{pre}{x}{suf}"))
    .collect()
}

pub async fn message_silent(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    let value = format!("{SILENT}{}", value.into());
    assert!(value.len() <= MAX_MESSAGE_LEN, "MAX LENGTH EXCEEDED!");
    message_helper(ctx, msg, value).await
}

#[expect(dead_code, reason = "We probably want to have the option later...")]
pub async fn message(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
) -> Result<(), SerenityError> {
    let value = value.into();
    assert!(value.len() <= MAX_MESSAGE_LEN, "MAX LENGTH EXCEEDED!");
    message_helper(ctx, msg, value).await
}

pub async fn message_helper(
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

pub async fn message_split<A: AsRef<str>, B: AsRef<str>>(
    ctx: &Context,
    msg: &Message,
    value: impl Into<String>,
    wrapper: Option<(A, B)>,
) -> Result<(), SerenityError> {
    let parts = match wrapper {
        Some((prefix, suffix)) => split_message_wrapped(value, prefix, suffix),
        None => split_message(value),
    };
    for string in parts {
        debug_assert!(string.len() <= MAX_MESSAGE_LEN);
        message_silent(&ctx, &msg, string).await?;
    }
    Ok(())
}
