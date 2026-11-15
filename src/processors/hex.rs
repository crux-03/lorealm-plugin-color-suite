use lorealm_plugin::{Result, Value, json, serde_json};
use rand::{RngExt, rngs::StdRng};
use serde::Deserialize;

use crate::utils::HEX_COMPONENTS;

#[derive(Default, Deserialize)]
pub struct RandomHexInputs {
    alpha: bool,
}

impl RandomHexInputs {
    pub fn from_inputs(inputs: Vec<(String, String)>) -> Result<Self> {
        let mut schema = Self::default();

        for (name, value) in inputs {
            match name.as_str() {
                "alpha" => {
                    let a: bool = serde_json::from_str::<bool>(&value)?.try_into()?;
                    schema.alpha = a;
                }
                &_ => return Err(format!("Invalid parameter: {name}").into()),
            }
        }

        Ok(schema)
    }
}

pub fn random_hex_proc(input: RandomHexInputs, rng: &mut StdRng) -> Result<Value> {
    let mut hex = String::new();
    let num_comps = if input.alpha { 8 } else { 6 };
    for _ in 0..num_comps {
        hex.push(HEX_COMPONENTS[rng.random_range(0..HEX_COMPONENTS.len())]);
    }

    Ok(Value::String(hex))
}

#[derive(Default, Deserialize)]
pub struct HexToRgbInputs {
    hex: String,
}

impl HexToRgbInputs {
    pub fn from_inputs(inputs: Vec<(String, String)>) -> Result<Self> {
        let mut schema = Self::default();

        for (name, value) in inputs {
            match name.as_str() {
                "hex" => schema.hex = serde_json::from_str(&value)?,
                &_ => return Err(format!("Invalid parameter: {name}").into()),
            }
        }

        Ok(schema)
    }
}

pub fn hex_to_rgb_proc(input: HexToRgbInputs) -> Result<Value> {
    let hex = input.hex.trim_start_matches('#');
    if !matches!(hex.len(), 6 | 8) || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("Invalid hex color: {}", input.hex).into());
    }

    let comp = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16);
    let mut rgb = json!({ "r": comp(0)?, "g": comp(2)?, "b": comp(4)? });
    if hex.len() == 8 {
        rgb["a"] = comp(6)?.into();
    }

    Ok(rgb)
}
