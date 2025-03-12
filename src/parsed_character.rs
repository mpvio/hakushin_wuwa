use crate::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedCharacter {
    pub id: i64,
    pub name: String,
    pub rarity: i64,
    pub weapon: i64,
    pub element: i64,
    pub tags: Vec<ParsedTag>,
    pub stats: ParsedStats,
    pub skills: BTreeMap<String, SkillTree>,
    pub chains: BTreeMap<String, ParsedChainDescription>, //move params to desc (created new ParsedChainDescription below)
    pub ascensions: BTreeMap<String, BTreeMap<String, i64>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedStats {
    #[serde(rename = "Life")]
    pub life: i64,
    #[serde(rename = "Atk")]
    pub atk: i64,
    #[serde(rename = "Def")]
    pub def: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedTag {
    pub name: String,
    pub desc: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTree {
    #[serde(rename = "ParentNodes")]
    pub parent_nodes: Vec<i64>,
    #[serde(rename = "NodeType")]
    pub node_type: i64,
    #[serde(rename = "Consume")]
    pub consume: BTreeMap<String, i64>,
    #[serde(rename = "Coordinate")]
    pub coordinate: i64,
    #[serde(rename = "UnLockCondition")]
    pub un_lock_condition: i64,
    #[serde(rename = "Skill")]
    pub skill: SkillVariant,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkillVariant {
    SkillS(SkillSmall),
    SkillL(SkillLarge),
    #[default] None
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillSmall {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String, //add params to this
    // #[serde(rename = "Param")]
    // pub param: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillLarge {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String, //add params to this
    // #[serde(rename = "Param")]
    // pub param: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Level")]
    pub level: BTreeMap<String, Level>,
    #[serde(rename = "Damage")]
    pub damage: BTreeMap<String, Damage>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    #[serde(rename = "RelatedProperty")]
    pub related_property: String,
    #[serde(rename = "Element")]
    pub element: i64,
    #[serde(rename = "ElementPower")]
    pub element_power: i64,
    #[serde(rename = "Energy")]
    pub energy: i64,
    #[serde(rename = "HardnessLv")]
    pub hardness_lv: i64,
    #[serde(rename = "RateLv")]
    pub rate_lv: String,
    #[serde(rename = "ToughLv")]
    pub tough_lv: i64,
    #[serde(rename = "Type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    #[serde(rename = "Format")]
    pub format: String, //move param to here. instead of "None", set this to "{0}"
    #[serde(rename = "Name")]
    pub name: String,
    // #[serde(rename = "Param")]
    // pub param: Vec<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedChainDescription {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String
}