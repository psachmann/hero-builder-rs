use anyhow::{Context, Result};

use crate::models::Hero;

/// Import a hero from a JSON string.
///
/// ## Arguments
/// The `json` argument is the JSON string to import.
///
/// ## Returns
/// The imported hero.
///
/// ## Errors
/// This function will return an error if the JSON string cannot be deserialized into a hero.
#[allow(dead_code)]
pub fn import_hero(json: &String) -> Result<Hero> {
    let hero = serde_json::from_str::<Hero>(json).with_context(|| "Failed to deserialize hero from JSON")?;

    Ok(hero)
}
