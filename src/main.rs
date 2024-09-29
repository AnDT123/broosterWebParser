mod helper;
use helper::stream::Stream;

mod dom;
use dom::parser::tokenizer;
use dom::entities::ENTITIES;
fn main() { 
        // Access the singleton dictionary anywhere in the program
        if let Some(entity) = ENTITIES.get("AMP") {
            println!("Character: {}, Codepoints: {:?}", entity.characters, entity.codepoints );
        }
        
        // Pass ENTITIES to another function
        use_entities();
}
fn use_entities() {
    if let Some(entity) = ENTITIES.get("AElig") {
        println!("In another function: Character: {}, Codepoints: {:?}", entity.characters, entity.codepoints);
    }
}
