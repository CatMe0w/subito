use std::error::Error;
use serde_json::Value;

pub fn parse(content: &Value) -> Result<&Value, Box<dyn Error>> {
    // todo
    // use strongly typed vec instead of value?
    Ok(content)
}