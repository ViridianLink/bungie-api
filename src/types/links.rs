use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HyperlinkReference {
    pub title: String,
    pub url: String,
}
