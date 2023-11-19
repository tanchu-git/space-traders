use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Ships {
    r#type: String,
    name: String,
    description: String,
    supply: String,
    activity: String,
    purchase_price: u32,
    frame: Frame,
    reactor: Reactor,
    engine: Engine,
    modules: Vec<Module>,
    mounts: Vec<Mount>,
    modifications_fee: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    symbol: String,
    name: String,
    description: String,
    module_slots: u16,
    mounting_points: u16,
    fuel_capacity: u16,
    requirements: Requirements,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Requirements {
    power: u16,
    crew: i16,
    slots: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    symbol: String,
    name: String,
    description: String,
    power_output: u16,
    requirements: Requirements,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Engine {
    symbol: String,
    name: String,
    description: String,
    speed: u16,
    requirements: Requirements,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Module {
    symbol: String,
    name: String,
    description: String,
    capacity: u16,
    requirements: Requirements,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct Mount {
    symbol: String,
    name: String,
    description: String,
    strength: u16,
    deposits: Vec<String>,
    requirements: Requirements,
}
