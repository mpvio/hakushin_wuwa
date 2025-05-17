use std::{fs::File, io::{self, BufReader, Seek, SeekFrom}};

use crate::{character_list::MinimalCharacterMap, parsed_character::ParsedCharacter};
use serde_json::json;
use serde_json_diff::Difference;

pub async fn write_to_file(character : ParsedCharacter){
    let title = format!("{}.json", character.name);

    if let Ok(mut file) = File::options()
    .read(true)
    .write(true)
    .create(true)
    .open(&title) {
        let reader = BufReader::new(&file);
        let saved_content: Result<ParsedCharacter, serde_json::Error> = serde_json::from_reader(reader);
        match saved_content {
            Ok(saved_char) => {
                let updated = compare_characters(&saved_char, &character).await;
                if updated {
                    write_character_to_file(&mut file, &character, &title, true);
                }
            },
            Err(_) => {
                //file didn't exist before
                write_character_to_file(&mut file, &character, &title, false);
            },
        }
    }
}

pub fn get_ids_from_user() -> String {
    let mut buffer: String = String::new();
    println!("Enter IDs: ");
    let stdin: io::Stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => {
            buffer
        },
        Err(_) => {
            String::new()
        },
    }
}

async fn compare_characters(old_char : &ParsedCharacter, new_char : &ParsedCharacter) -> bool {
    let old_char_json = json!(old_char);
    let new_char_json = json!(new_char);

    match serde_json_diff::values(old_char_json, new_char_json) {
        Some(differences) => {
            println!("{differences:#?}");
            write_diff_to_file(&differences, &old_char.name);
            true
        },
        None => {
            false
        },
    }
}

fn write_diff_to_file(character : &Difference, name: &String){
    let title = format!("Changes to {}.json", name);

    if let Ok(file) = File::options()
    //.read(true)
    .write(true)
    .truncate(true)
    .create(true)
    .open(&title) {
        //let reader = BufReader::new(&file);
        let write_outcome = serde_json::to_writer_pretty(file, &character);
        match write_outcome {
            Ok(_) => {
                println!("{title} created.");
            },
            Err(_) => {
                println!("Error with {title}.");
            },
        }
    }
}

fn write_character_to_file(file: &mut File, character: &ParsedCharacter, title: &String, update: bool){
    let _ = file.seek(SeekFrom::Start(0));
    match serde_json::to_writer_pretty(file, &character) {
        Ok(_) => {
            if update {
                println!("{title} updated.");
            } else {
                println!("{title} created.");
            }
        },
        Err(err) => {
            println!("{:#?}", err);
        },
    }
}

pub fn write_character_list_to_file(map: &MinimalCharacterMap){
    let path = "characters.json";
    let mut file = File::create(path).unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    match serde_json::to_writer_pretty(file, &map) {
        Ok(_) => {
            println!("{path} created.");
            }
        ,
        Err(err) => {
            println!("{:#?}", err);
        },
    }
}