use lorealm_plugin::{Result, Value, json, serde_json};
use rand::{RngExt, rngs::StdRng};
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct RgbToHexInputs {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

impl RgbToHexInputs {
    pub fn from_inputs(inputs: Vec<(String, String)>) -> Result<Self> {
        let mut schema = Self::default();

        for (name, value) in inputs {
            match name.as_str() {
                "r" => schema.r = serde_json::from_str(&value)?,
                "g" => schema.g = serde_json::from_str(&value)?,
                "b" => schema.b = serde_json::from_str(&value)?,
                "a" => schema.a = serde_json::from_str(&value)?,
                &_ => return Err(format!("Invalid parameter: {name}").into()),
            }
        }

        Ok(schema)
    }
}

pub fn rgb_to_hex_proc(input: RgbToHexInputs) -> Result<Value> {
    let mut hex = format!("{:02x}{:02x}{:02x}", input.r, input.g, input.b);
    if let Some(a) = input.a {
        hex.push_str(&format!("{a:02x}"));
    }

    Ok(Value::String(hex))
}

#[derive(Default, Deserialize)]
pub struct RandomRgbInputs {
    alpha: bool,
}

impl RandomRgbInputs {
    pub fn from_inputs(inputs: Vec<(String, String)>) -> Result<Self> {
        let mut schema = Self::default();

        for (name, value) in inputs {
            match name.as_str() {
                "alpha" => schema.alpha = serde_json::from_str(&value)?,
                &_ => return Err(format!("Invalid parameter: {name}").into()),
            }
        }

        Ok(schema)
    }
}

pub fn random_rgb_proc(input: RandomRgbInputs, rng: &mut StdRng) -> Result<Value> {
    let mut rgb =
        json!({ "r": rng.random::<u8>(), "g": rng.random::<u8>(), "b": rng.random::<u8>() });
    if input.alpha {
        rgb["a"] = rng.random::<u8>().into();
    }

    Ok(rgb)
}
