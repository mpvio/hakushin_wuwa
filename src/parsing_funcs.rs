use std::collections::BTreeMap;
use serde_json::Value;
use regex::{Captures, Regex};

use crate::{character::{ChainDescription, Consume, N90}, parsed_character::{self, Damage, Level, ParsedChainDescription, ParsedStats, ParsedTag, SkillLarge, SkillSmall, SkillTree, SkillVariant}};

pub fn parse_character_tag(tags : &serde_json::Value) -> Vec<ParsedTag>{
    let values = tags.as_object().unwrap().values();
    let mut new_tags : Vec<ParsedTag> = Vec::new();
    for v in values {
        let value = v.as_object().unwrap();
        let name = value.get("Name").unwrap().as_str().unwrap().to_string();
        let desc = value.get("Desc").unwrap().as_str().unwrap().to_string();
        let new_tag = ParsedTag {
            name,
            desc
        };

        new_tags.push(new_tag);
    }
    return new_tags;
}

fn add_or_update_map(map : &mut BTreeMap<i64, i64>, key : &i64, value : &i64){
    match map.get(&key) {
        Some(val) => {
            map.insert(*key, val + value);
        },
        None => {
            map.insert(*key, *value);
        },
    }
}

fn consume_costs (consume : &Vec<Value>) -> BTreeMap<i64, i64> {
    let mut total_costs = BTreeMap::<i64, i64>::new();
    for con in consume {
        let key = con.get("Key").unwrap().as_i64().unwrap();
        let value = con.get("Value").unwrap().as_i64().unwrap();
        add_or_update_map(&mut total_costs, &key, &value);
    }
    return total_costs;
}

fn parse_format(test_text : Option<&str>) -> String {
    match test_text {
        Some(text) => {
            return text.to_string();
        },
        None => {
            return "{0}".to_string();
        },
    }
}

fn parse_level (level : &serde_json::Map<String, Value>) -> BTreeMap<std::string::String, Level> {
    //enter map
    let mut level_map : BTreeMap<String, Level> = BTreeMap::new();
    for (k, v) in level { 
        //key is 1, 2, 3, etc.
        //value is the level object (format, name, params)
        let level_obj = v.as_object().unwrap();

        let mut format : String = parse_format(level_obj.get("Format").unwrap().as_str());
        let name = level_obj.get("Name").unwrap().as_str().unwrap().to_string();

        //first level array
        let param1 = level_obj.get("Param").unwrap().as_array().unwrap();

        let param_list = param1.get(0).unwrap().as_array().unwrap();
        format = parse_level_regex(format, param_list);

        let new_level = Level {
            format,
            name
        };
        level_map.insert(k.clone(), new_level);        
    }
    return level_map;
}

fn parse_consume_map (consumes : &serde_json::Map<String, Value>) -> BTreeMap<i64, i64> {
    let mut total_costs_map = BTreeMap::<i64, i64>::new();
    for (_k, v) in consumes {
        let cons_vec = v.as_array().unwrap();
        let array_cost = consume_costs(cons_vec);

        for (obj_id, obj_cost) in array_cost {

            add_or_update_map(&mut total_costs_map, &obj_id, &obj_cost);
        }
    }
    return total_costs_map;
}

fn parse_damage(damage: &serde_json::Map<String, Value>) -> BTreeMap<std::string::String, Damage> {
    let mut damage_map = BTreeMap::<String, Damage>::new();
    for (k, v) in damage {
        //k is key
        //value is damage object
        let dam_obj = v.as_object().unwrap();

        let rate_lv_arr = dam_obj.get("RateLv").unwrap().as_array().unwrap();
        //let mut rate_lv = Vec::<f64>::new();
        let n0 = rate_lv_arr.get(0).unwrap().as_i64().unwrap() as f64/100.0;
        let n9 = rate_lv_arr.get(9).unwrap().as_i64().unwrap() as f64/100.0;
        let rate_lv = format!("[{n0}|{n9}]%");
        // for rl in rate_lv_arr {
        //     rate_lv.push(rl.as_i64().unwrap());
        // }

        let damage_obj = Damage {
            element: dam_obj.get("Element").unwrap().as_i64().unwrap(),
            element_power: dam_obj.get("ElementPower").unwrap().as_i64().unwrap(),
            energy: dam_obj.get("Energy").unwrap().as_i64().unwrap(),
            hardness_lv: dam_obj.get("HardnessLv").unwrap().as_i64().unwrap(),
            rate_lv,
            related_property: dam_obj.get("RelatedProperty").unwrap().as_str().unwrap().to_string(),
            tough_lv: dam_obj.get("ToughLv").unwrap().as_i64().unwrap(),
            type_field: dam_obj.get("Type").unwrap().as_i64().unwrap()
        };
        damage_map.insert(k.clone(), damage_obj);
    }
    return damage_map;
}

fn parse_skill (skill : &serde_json::Map<String, Value>) -> (SkillVariant, BTreeMap::<i64,i64>) {
    let is_small_skill = skill.len() < 5;

    let name = skill.get("Name").unwrap().as_str().unwrap().to_string();
    let mut desc = skill.get("Desc").unwrap().as_str().unwrap().to_string();
    let param = skill.get("Param").unwrap().as_array().unwrap();
    desc = parse_skill_regex(desc, param);

    if is_small_skill {
        let parsed_skill = SkillSmall {
            name,
            desc
        };
        return (parsed_character::SkillVariant::SkillS(parsed_skill), BTreeMap::<i64, i64>::new());
    } else {
        let type_field = skill.get("Type").unwrap().as_str().unwrap().to_string();

        //level
        let level: &serde_json::Map<String, Value> = skill.get("Level").unwrap().as_object().unwrap();
        let level_map = parse_level(level);

        //consume <- collect values in a map!
        let consumes: &serde_json::Map<String, Value> = skill.get("Consume").unwrap().as_object().unwrap();
        let consume = parse_consume_map(consumes);

        //damage
        let damage_serde = skill.get("Damage").unwrap().as_object().unwrap();
        let damage = parse_damage(damage_serde);

        let parsed_skill = SkillLarge {
            name,
            desc,
            type_field,
            level: level_map,
            damage
        };
        return (parsed_character::SkillVariant::SkillL(parsed_skill), consume);
    }
}

pub async fn parse_character_skilltrees(skilltrees : &BTreeMap<String, serde_json::Value>) -> (BTreeMap<std::string::String, SkillTree>, std::option::Option<Value>) {
    let mut skill_map = BTreeMap::<String, SkillTree>::new();
    let item_map = get_item_object().await;

    for (main_key, value) in skilltrees {
        let skilltree = value.as_object().unwrap();

        //extract Consume
        let consume = skilltree.get("Consume").unwrap().as_array().unwrap();
        let mut consume_map = consume_costs(consume);
        //coordinate
        let coord = skilltree.get("Coordinate").unwrap().as_i64().unwrap();
        //nodetype
        let node = skilltree.get("NodeType").unwrap().as_i64().unwrap();
        //parentnode
        let parent = skilltree.get("ParentNodes").unwrap().as_array().unwrap();
        let mut parent_vec : Vec<i64> = Vec::new();
        for p in parent {
            parent_vec.push(p.as_i64().unwrap());
        }
        //unlock
        let unlock = skilltree.get("UnLockCondition").unwrap().as_i64().unwrap();
        //skill (variable)
        let skill: &serde_json::Map<String, Value> = skilltree.get("Skill").unwrap().as_object().unwrap();
        let (new_skill, skill_consume_map) = parse_skill(skill);

        for (key, value) in skill_consume_map {
            add_or_update_map(&mut consume_map, &key, &value);
        }

        let named_map = match_item_names(&item_map, &consume_map);

        let new_skill_tree = SkillTree {
            parent_nodes: parent_vec,
            node_type: node,
            consume: named_map,
            coordinate: coord,
            un_lock_condition: unlock,
            skill: new_skill
        };
        skill_map.insert(main_key.clone(), new_skill_tree);
    }
    return (skill_map, item_map);
}

pub fn parse_chains (chains: BTreeMap<String, ChainDescription>) -> BTreeMap<String, ParsedChainDescription> {
    let mut parsed_chains = BTreeMap::<String, ParsedChainDescription>::new();
    for (key, chain) in chains {
        let desc = parse_desc_regex(chain.desc, &chain.param);
        parsed_chains.insert(key, ParsedChainDescription {
            name: chain.name,
            desc
        });
    }
    return parsed_chains;
}

async fn get_item_object () -> Option<Value> {
    let url = "https://api.hakush.in/ww/data/en/item.json";
    if let Ok(get_url) = reqwest::Url::parse(url) {
        if let Ok(response) = reqwest::get(get_url).await {
            if response.status() == reqwest::StatusCode::OK {
                if let Ok(result) = response.json::<Value>().await {
                    return Some(result);
                }
            }
        }
    }
    return None;
}

pub fn parse_stats (stats : N90) -> ParsedStats {
    let life = stats.life.round() as i64;
    let atk = stats.atk.round() as i64;
    let def = stats.def.round() as i64;
    return ParsedStats{
        life,
        atk,
        def
    };
}

pub fn parse_ascensions (item_map : &Option<Value>, ascensions : BTreeMap<String, Vec<Consume>>) -> BTreeMap<String, BTreeMap<String, i64>> {
    let mut all_asc_map = BTreeMap::<String, BTreeMap::<String, i64>>::new();
    for (key, asc) in ascensions {
        let mut this_asc_map = BTreeMap::<String, i64>::new();
        for consume in asc {
            match item_map {
                Some(proper_item_map) => {
                    this_asc_map.insert(match_value(proper_item_map, &consume.key), consume.value);
                },
                None => {
                    this_asc_map.insert(consume.key.to_string(), consume.value);
                },
            } 
        }
        all_asc_map.insert(key, this_asc_map);
    }
    return all_asc_map;
}

fn match_value (items: &Value, key : &i64) -> String {
    let key_str = key.to_string();
    match items.get(&key_str) {
        Some(item_value) => {
            return item_value.get("name").unwrap().as_str().unwrap().to_string();
        },
        None => {
           return key_str;
        },
    }
}

fn match_item_names (items: &Option<Value>, map : &BTreeMap<i64,i64>) -> BTreeMap<std::string::String, i64> {
    let mut named_map: BTreeMap<String, i64> = BTreeMap::<String, i64>::new();
    match items {
        Some(item_map) => {
            for (k, v) in map {
                named_map.insert(match_value(item_map, k), *v);
            }
        },
        None => {
            for (k, v) in map {
                named_map.insert(k.to_string(), *v);
            }
        },
    }
    return named_map;
}

fn parse_desc_regex(desc: String, param: &Vec<String>) -> String {
    let re = Regex::new(r"\{([0-9]+)\}").unwrap();
    let haystack = &desc;
    let interpolated = re.replace_all(haystack, |caps: &Captures| {
        let Ok(index) = caps[1].parse::<usize>() else {
            return caps[1].to_string();
        };
        let Some(int) = param.get(index) else {
            return caps[1].to_string();
        };
        int.to_string()
    });

    let html_regexes = [r"<size=[0-9]+>", r"<color=[a-zA-Z]+>", r"<\/size>", r"<\/color>"];
    let mut new_desc = interpolated.to_string();

    for hre in html_regexes {
        let r = Regex::new(hre).unwrap();
        new_desc = r.replace_all(&new_desc, "").to_string();
    }

    new_desc = Regex::new(r"Attack\n").unwrap().replace_all(&new_desc, "Attack: ").to_string();
    new_desc = Regex::new(r"Counter\n").unwrap().replace_all(&new_desc, "Counter: ").to_string();
    new_desc = Regex::new(r"\n-").unwrap().replace_all(&new_desc, " -").to_string();
    new_desc = Regex::new(r"\.\n").unwrap().replace_all(&new_desc, ".").to_string();
    new_desc = Regex::new(r" \n").unwrap().replace_all(&new_desc, " ").to_string();
    new_desc = Regex::new(r"\n").unwrap().replace_all(&new_desc, ": ").to_string();

    return new_desc;
}

fn parse_skill_regex(desc: String, param: &Vec<Value>) -> String {
    let mut new_param = Vec::<String>::new();
    for p in param {
        new_param.push(p.as_str().unwrap().to_string());
    }
    parse_desc_regex(desc, &new_param)
}

fn parse_level_regex (level_format: String, params: &Vec<Value>) -> String {
    let re = Regex::new(r"\{([0-9]+)\}").unwrap();
    let haystack = &level_format;
    let n0 = params.get(0).unwrap().as_str().unwrap().to_string();
    let formatted_text = match params.get(9) {
        Some(n9_value) => {
            let n9 = n9_value.as_str().unwrap().to_string();
            if n9.eq(&n0) {
                format!("{n0}")
            } else {
                format!("[{n0}|{n9}]")
            }
        },
        None => {
            format!("{n0}")
        },
    };

    let new_level_format = re.replace_all(haystack, formatted_text);
    return new_level_format.to_string();
}