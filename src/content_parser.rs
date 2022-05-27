use std::error::Error;
use serde_json::Value;

use crate::Content;

pub fn parse(content: &Value) -> Result<Vec<Content>, Box<dyn Error>> {
    // todo
    // use strongly typed vec instead of value?
    Ok(vec![])
}