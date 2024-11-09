use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DestinyColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
