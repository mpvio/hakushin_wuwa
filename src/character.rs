use std::collections::BTreeMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Rarity")]
    pub rarity: i64,
    #[serde(rename = "Weapon")]
    pub weapon: i64,
    #[serde(rename = "Element")]
    pub element: i64,
    #[serde(rename = "Tag")]
    pub tag: serde_json::Value, //handled
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Stats")]
    pub stats: Stats,
    #[serde(rename = "SkillTrees")]
    pub skill_trees: BTreeMap<String, serde_json::Value>,
    #[serde(rename = "Chains")]
    pub chains: BTreeMap<String, ChainDescription>,
    #[serde(rename = "Ascensions")]
    pub ascensions: BTreeMap<String, Vec<Consume>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(rename = "6")]
    pub n6: N610,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N610 {
    #[serde(rename = "90")]
    pub n90: N90,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N90 {
    #[serde(rename = "Life")]
    pub life: f64,
    #[serde(rename = "Atk")]
    pub atk: f64,
    #[serde(rename = "Def")]
    pub def: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTrees {
    #[serde(rename = "1")]
    pub n1: serde_json::Value,
    #[serde(rename = "2")]
    pub n2: serde_json::Value,
    #[serde(rename = "3")]
    pub n3: serde_json::Value,
    #[serde(rename = "4")]
    pub n4: serde_json::Value,
    #[serde(rename = "5")]
    pub n5: serde_json::Value,
    #[serde(rename = "6")]
    pub n6: serde_json::Value,
    #[serde(rename = "7")]
    pub n7: serde_json::Value,
    #[serde(rename = "8")]
    pub n8: serde_json::Value,
    #[serde(rename = "9")]
    pub n9: serde_json::Value,
    #[serde(rename = "10")]
    pub n10: serde_json::Value,
    #[serde(rename = "11")]
    pub n11: serde_json::Value,
    #[serde(rename = "12")]
    pub n12: serde_json::Value,
    #[serde(rename = "13")]
    pub n13: serde_json::Value,
    #[serde(rename = "14")]
    pub n14: serde_json::Value,
    #[serde(rename = "15")]
    pub n15: serde_json::Value,
    #[serde(rename = "16")]
    pub n16: serde_json::Value
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillBreakdown {
    #[serde(rename = "ParentNodes")]
    pub parent_nodes: Vec<i64>,
    #[serde(rename = "NodeType")]
    pub node_type: i64,
    #[serde(rename = "Coordinate")]
    pub coordinate: i64,
    #[serde(rename = "UnLockCondition")]
    pub un_lock_condition: i64,
    #[serde(rename = "Consume")]
    pub consume: serde_json::Value,
    #[serde(rename = "Skill")]
    pub skill: SkillRaw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consume {
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRaw {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "Param")]
    pub param: Vec<String>,
    #[serde(rename = "Type")]
    pub type_field: Option<String>,
    #[serde(rename = "Level")]
    pub level: Option<serde_json::Value>,
    #[serde(rename = "Consume")]
    pub consume: Vec<serde_json::Value>,
    #[serde(rename = "Damage")]
    pub damage: Option<serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill_ {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "Param")]
    pub param: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chains {
    #[serde(rename = "1")]
    pub n1: ChainDescription,
    #[serde(rename = "2")]
    pub n2: ChainDescription,
    #[serde(rename = "3")]
    pub n3: ChainDescription,
    #[serde(rename = "4")]
    pub n4: ChainDescription,
    #[serde(rename = "5")]
    pub n5: ChainDescription,
    #[serde(rename = "6")]
    pub n6: ChainDescription
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainDescription {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "Param")]
    pub param: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ascensions {
    #[serde(rename = "1")]
    pub n1: Vec<Consume>,
    #[serde(rename = "2")]
    pub n2: Vec<Consume>,
    #[serde(rename = "3")]
    pub n3: Vec<Consume>,
    #[serde(rename = "4")]
    pub n4: Vec<Consume>,
    #[serde(rename = "5")]
    pub n5: Vec<Consume>,
    #[serde(rename = "6")]
    pub n6: Vec<Consume>
}

/*
n111
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n111 {
    #[serde(rename = "ParentNodes")]
    pub parent_nodes: Vec<Value>,
    #[serde(rename = "NodeType")]
    pub node_type: i64,
    #[serde(rename = "Coordinate")]
    pub coordinate: i64,
    #[serde(rename = "UnLockCondition")]
    pub un_lock_condition: i64,
    #[serde(rename = "Consume")]
    pub consume: Vec<Consume>,
    #[serde(rename = "Skill")]
    pub skill: Skill,
}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consume {
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "Value")]
    pub value: i64,
}

n162
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n162 {
    #[serde(rename = "ParentNodes")]
    pub parent_nodes: Vec<i64>,
    #[serde(rename = "NodeType")]
    pub node_type: i64,
    #[serde(rename = "Coordinate")]
    pub coordinate: i64,
    #[serde(rename = "UnLockCondition")]
    pub un_lock_condition: i64,
    #[serde(rename = "Consume")]
    pub consume: Vec<Consume24>,
    #[serde(rename = "Skill")]
    pub skill: Skill16,
}

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consume24 {
    #[serde(rename = "Key")]
    pub key: i64,
    #[serde(rename = "Value")]
    pub value: i64,
}

*/