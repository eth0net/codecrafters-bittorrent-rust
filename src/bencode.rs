use std::str::FromStr;

use anyhow::Context;

pub fn decode(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let (first, _) = encoded_value.split_at(1);
    match first {
        "i" => decode_integer(encoded_value),
        _ => decode_string(encoded_value),
    }
}

fn decode_integer(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    // Example: "i52e" -> 52
    let (_, rest) = encoded_value.split_once('i')
        .with_context(|| "integer starts with i")?;
    let (rest, _) = rest.split_once('e')
        .with_context(|| "integer end with e")?;
    let value = i64::from_str(rest).with_context(|| "parsing int")?;
    Ok(serde_json::Value::from(value))
}

fn decode_string(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').with_context(|| "expected ':'")?;
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().with_context(|| "expected number before `:`")?;
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    Ok(serde_json::Value::String(string.to_string()))
}
