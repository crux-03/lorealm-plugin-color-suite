use lorealm_plugin::{
    Param, Plugin, ProcessorCall, ProcessorSignature, Result, Value, host::register_processor,
};
use rand::rngs::StdRng;

use crate::processors::hex::{HexToRgbInputs, RandomHexInputs, hex_to_rgb_proc, random_hex_proc};
use crate::processors::named::{
    NamedColorInputs, RandomColorNameInputs, named_color_proc, random_color_name_proc,
};
use crate::processors::rgb::{RandomRgbInputs, RgbToHexInputs, random_rgb_proc, rgb_to_hex_proc};

mod processors;
pub mod utils;

struct ColorSuite {
    rng: StdRng,
}

impl Plugin for ColorSuite {
    fn init() -> Result<Self> {
        register_processor(&ProcessorSignature {
            name: "random_hex".into(),
            description: "Generates a random hex color".into(),
            returns: lorealm_plugin::ValueType::String,
            properties: vec![Param {
                name: "alpha".into(),
                description: "Whether to include the Alpha component".into(),
                value_type: Some(lorealm_plugin::ValueType::Bool),
                required: false,
            }],
        })?;
        register_processor(&ProcessorSignature {
            name: "hex_to_rgb".into(),
            description: "Converts a hex color to an RGB object".into(),
            returns: lorealm_plugin::ValueType::Object,
            properties: vec![Param {
                name: "hex".into(),
                description: "The hex color, 6 or 8 digits, with or without a leading #".into(),
                value_type: Some(lorealm_plugin::ValueType::String),
                required: true,
            }],
        })?;
        register_processor(&ProcessorSignature {
            name: "random_rgb".into(),
            description: "Generates a random RGB object".into(),
            returns: lorealm_plugin::ValueType::Object,
            properties: vec![Param {
                name: "alpha".into(),
                description: "Whether to include the Alpha component".into(),
                value_type: Some(lorealm_plugin::ValueType::Bool),
                required: false,
            }],
        })?;
        register_processor(&ProcessorSignature {
            name: "rgb_to_hex".into(),
            description: "Converts RGB components to a hex color".into(),
            returns: lorealm_plugin::ValueType::String,
            properties: ["r", "g", "b", "a"]
                .into_iter()
                .map(|c| Param {
                    name: c.into(),
                    description: format!("The {c} component, 0-255"),
                    value_type: Some(lorealm_plugin::ValueType::Number),
                    required: c != "a",
                })
                .collect(),
        })?;
        register_processor(&ProcessorSignature {
            name: "named_color".into(),
            description: "Looks up the hex color of a CSS color name".into(),
            returns: lorealm_plugin::ValueType::String,
            properties: vec![Param {
                name: "name".into(),
                description: "The color name, such as red or light sea green".into(),
                value_type: Some(lorealm_plugin::ValueType::String),
                required: true,
            }],
        })?;
        register_processor(&ProcessorSignature {
            name: "random_color_name".into(),
            description: "Picks a random color name".into(),
            returns: lorealm_plugin::ValueType::String,
            properties: vec![Param {
                name: "specific".into(),
                description: "Whether to include more specific colors, such as light sea green"
                    .into(),
                value_type: Some(lorealm_plugin::ValueType::Bool),
                required: false,
            }],
        })?;
        Ok(Self {
            rng: rand::make_rng(),
        })
    }

    fn processor(&mut self, call: ProcessorCall) -> Result<Value> {
        match call.name.as_str() {
            "random_hex" => {
                let input = RandomHexInputs::from_inputs(call.properties)?;
                random_hex_proc(input, &mut self.rng)
            }
            "hex_to_rgb" => hex_to_rgb_proc(HexToRgbInputs::from_inputs(call.properties)?),
            "random_rgb" => {
                let input = RandomRgbInputs::from_inputs(call.properties)?;
                random_rgb_proc(input, &mut self.rng)
            }
            "rgb_to_hex" => rgb_to_hex_proc(RgbToHexInputs::from_inputs(call.properties)?),
            "random_color_name" => {
                let input = RandomColorNameInputs::from_inputs(call.properties)?;
                random_color_name_proc(input, &mut self.rng)
            }
            "named_color" => named_color_proc(NamedColorInputs::from_inputs(call.properties)?),
            &_ => return Err(format!("Invalid processor: {}", call.name).into()),
        }
    }
}

lorealm_plugin::export!(ColorSuite);
