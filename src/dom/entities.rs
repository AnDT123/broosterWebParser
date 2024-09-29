use serde::{Deserialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use once_cell::sync::Lazy; // Use sync::Lazy for thread-safe access

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub codepoints: Vec<u32>,
    pub characters: String,
}

pub type EntityMap = HashMap<String, Entity>;

pub static ENTITIES: Lazy<EntityMap> = Lazy::new(|| {
    load_entities("./src/dom/entities.json").expect("Failed to load entities.json")
});

fn load_entities(file_path: &str) -> Result<EntityMap> {
    let file_content = fs::read_to_string(file_path).unwrap();
    let mut entities: EntityMap = serde_json::from_str(&file_content)?;

    entities = entities.into_iter()
        .map(|(k, v)| {
            let clean_key = k.trim_start_matches('&').to_string();
            (clean_key, v)
        })
        .collect();

    Ok(entities)
}
