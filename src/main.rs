use std::collections::BTreeMap;

use character::Character;
use parsed_character::ParsedCharacter;
use parsing_funcs::{parse_ascensions, parse_chains, parse_character_skilltrees, parse_character_tag, parse_stats};
use read_and_write_funcs::{get_ids_from_user, write_to_file};

pub mod character;
pub mod parsed_character;
pub mod parsing_funcs;
pub mod read_and_write_funcs;

#[tokio::main]
async fn main() {
    let inputs: String = get_ids_from_user();
    let ids : Vec<&str> = inputs.split_ascii_whitespace().collect();
    for id in ids {
        let character = character_api_access(id).await;
        //println!("{:#?}", character);
        write_to_file(character).await;
    }
}

async fn character_api_access(char_id : &str) -> ParsedCharacter {
    let base_url = format!("https://api.hakush.in/ww/data/en/character/{}.json",char_id);

    if let Ok(get_url) = reqwest::Url::parse(&base_url) {
        let response = reqwest::get(get_url).await;
        if let Ok(resp) = response {
            if resp.status() == reqwest::StatusCode::OK {
                let parsed_result = resp.json::<Character>().await;
                match parsed_result {
                    Ok(result) => {
                        //convert Value to tags
                        let tags = parse_character_tag(&result.tag);
                        //handle skill tree
                        let (new_tree, item_map) = parse_character_skilltrees(&result.skill_trees).await;

                        //println!("{new_tree:#?}");

                        let chains = parse_chains(result.chains);
                        //println!("{chains:#?}\n");

                        let ascensions: BTreeMap<String, Vec<character::Consume>> = result.ascensions;
                        let new_ascensions: BTreeMap<String, BTreeMap<String, i64>> = parse_ascensions(&item_map, ascensions);
                        //println!("{new_ascensions:#?}");

                        let stats = parse_stats(result.stats.n6.n90);

                        let new_character = ParsedCharacter {
                            id: result.id,
                            name: result.name,
                            rarity: result.rarity,
                            weapon: result.weapon,
                            element: result.element,
                            tags,
                            stats,
                            skills: new_tree,
                            chains,
                            ascensions: new_ascensions
                        };
                        return new_character;
                    },
                    Err(err) => {
                        println!("{:#?}", err);
                    },
                }
                panic!("Response not OK.");
            }
            panic!("No response.");
        }
        panic!("URL get failed.");
    }
    panic!("Something wrong!");
}



