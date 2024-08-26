use anyhow::{Context, Ok, Result};
use leptos::SignalGet;

use crate::models::Hero;

/// Export a hero to a JSON string.
///
/// ## Arguments
/// The `hero` argument is the hero to export.
///
/// ## Returns
/// A tuple containing the hero's name and the JSON string.
///
/// ## Errors
/// This function will return an error if the hero cannot be serialized into JSON.
#[allow(dead_code)]
pub fn export_hero(hero: &Hero) -> Result<(String, String)> {
    let hero_json =
        serde_json::to_string(hero).with_context(|| "Failed to serialize hero into JSON")?;

    Ok((hero.name.get(), hero_json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Hero;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_export_hero() {
        let hero = Hero::default();
        let (name, json) = export_hero(&hero).unwrap();

        assert_eq!(name, "");
        assert_eq!(json, "{\"id\":\"00000000-0000-0000-0000-000000000000\",\"name\":\"\",\"hero_rank\":1,\"size_class\":0,\"available_xp\":0,\"spent_xp\":0}");
    }
}
