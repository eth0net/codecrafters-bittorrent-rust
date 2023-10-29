use std::str::FromStr;

use anyhow::Context;

pub fn decode(encoded_value: &str) -> anyhow::Result<(serde_json::Value, &str)> {
    let first = &encoded_value[0..1];
    match first {
        "i" => decode_integer(encoded_value),
        "l" => decode_list(encoded_value),
        _ => decode_string(encoded_value),
    }
}

fn decode_integer(encoded_value: &str) -> anyhow::Result<(serde_json::Value, &str)> {
    // Example: "i52e" -> 52
    let (_, value) = encoded_value.split_once('i')
        .with_context(|| "integer starts with i")?;
    let (value, rest) = value.split_once('e')
        .with_context(|| "integer end with e")?;
    let value = i64::from_str(value).with_context(|| "parsing int")?;
    Ok((serde_json::Value::from(value), rest))
}

fn decode_list(encoded_value: &str) -> anyhow::Result<(serde_json::Value, &str)> {
    // Example: "l5:helloi52ee" -> ["hello",52]
    let mut encoded_value = &encoded_value[1..];
    let mut values = Vec::new();

    while !encoded_value.is_empty() && encoded_value.ne("e") {
        let (value, rest) = decode(encoded_value)?;
        values.push(value);
        encoded_value = rest;
    }

    Ok((serde_json::Value::from(values), encoded_value))
}

fn decode_string(encoded_value: &str) -> anyhow::Result<(serde_json::Value, &str)> {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').with_context(|| "expected ':'")?;
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().with_context(|| "expected number before `:`")?;
    let end_index = colon_index + 1 + number as usize;
    let string = &encoded_value[colon_index + 1..end_index];
    let rest = &encoded_value[end_index..];
    Ok((serde_json::Value::from(string), rest))
}
