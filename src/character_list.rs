use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinimalCharacter {
    pub en: String
}

pub type MinimalCharacterMap = BTreeMap<String, MinimalCharacter>;