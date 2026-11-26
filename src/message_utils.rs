use std::num::NonZero;

use serenity::{
    Error as SerenityError,
    all::{Context, Message, ModelError},
};

use crate::MAX_MESSAGE_LEN;

const fn maxlen() -> NonZero<usize> {
    unsafe { NonZero::new(MAX_MESSAGE_LEN).unwrap_unchecked() }
}

pub fn split_message(input: impl Into<String>, len: NonZero<usize>) -> Vec<String> {
    let string: String = input.into();
    let mut parts = Vec::new();
    let mut iter = string.chars();
    loop {
        let mut string_val = String::with_capacity(len.get());
        for _ in 0..len.get() {
            match iter.next() {
                Some(c) => string_val.push(c),
                None => break,
            }
        }
        parts.push(string_val);
        return parts;
    }
}

pub fn split_message_wrapped(
    input: impl Into<String>,
    len: NonZero<usize>,
    prefix: impl AsRef<str>,
    suffix: impl AsRef<str>,
) -> Vec<String> {
    let pre: &str = prefix.as_ref();
    let suf: &str = suffix.as_ref();
    split_message(
        input,
        NonZero::new(len.get().strict_sub(pre.len()).strict_sub(suf.len())).unwrap(),
    )
    .into_iter()
    .map(|x| format!("{pre}{x}{suf}"))
    .collect()
}

pub async fn message(
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
        Some((prefix, suffix)) => split_message_wrapped(value, maxlen(), prefix, suffix),
        None => split_message(value, maxlen()),
    };
    for string in parts {
        assert!(string.len() <= MAX_MESSAGE_LEN);
        message(&ctx, &msg, string).await?;
    }
    Ok(())
}
