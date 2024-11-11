use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HyperlinkReference {
    pub title: String,
    pub url: String,
}
