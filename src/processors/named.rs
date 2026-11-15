use lorealm_plugin::{Result, Value, serde_json};
use rand::{RngExt, rngs::StdRng};
use serde::Deserialize;

use crate::utils::{BASIC_COLORS, SPECIFIC_COLORS};

#[derive(Default, Deserialize)]
pub struct NamedColorInputs {
    name: String,
}

impl NamedColorInputs {
    pub fn from_inputs(inputs: Vec<(String, String)>) -> Result<Self> {
        let mut schema = Self::default();

        for (name, value) in inputs {
            match name.as_str() {
                "name" => schema.name = serde_json::from_str(&value)?,
                &_ => return Err(format!("Invalid parameter: {name}").into()),
            }
        }

        Ok(schema)
    }
}

pub fn named_color_proc(input: NamedColorInputs) -> Result<Value> {
    let name = input.name.to_lowercase().replace(' ', "");
    let (_, hex) = BASIC_COLORS
        .iter()
        .chain(SPECIFIC_COLORS.iter())
        .find(|(n, _)| n.replace(' ', "") == name)
        .ok_or_else(|| format!("Unknown color: {}", input.name))?;

    Ok(Value::String(hex.to_string()))
}

#[derive(Default, Deserialize)]
pub struct RandomColorNameInputs {
    specific: bool,
}

impl RandomColorNameInputs {
    pub fn from_inputs(inputs: Vec<(String, String)>) -> Result<Self> {
        let mut schema = Self::default();

        for (name, value) in inputs {
            match name.as_str() {
                "specific" => schema.specific = serde_json::from_str(&value)?,
                &_ => return Err(format!("Invalid parameter: {name}").into()),
            }
        }

        Ok(schema)
    }
}

pub fn random_color_name_proc(input: RandomColorNameInputs, rng: &mut StdRng) -> Result<Value> {
    let count = if input.specific {
        BASIC_COLORS.len() + SPECIFIC_COLORS.len()
    } else {
        BASIC_COLORS.len()
    };
    let i = rng.random_range(0..count);
    let (name, _) = if i < BASIC_COLORS.len() {
        BASIC_COLORS[i]
    } else {
        SPECIFIC_COLORS[i - BASIC_COLORS.len()]
    };

    Ok(Value::String(name.into()))
}
