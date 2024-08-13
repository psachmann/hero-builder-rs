use leptos::*;

use super::{attributes::DerivedAttributes, hero::Hero};

/// The skill of a hero.
/// 0: The memoized value of the skill.
/// 1: The current value of invested points in the skill.
/// 2: Temporary points that can be added to the skill.
pub type Skill = (Memo<i32>, RwSignal<i32>, RwSignal<i32>);

#[derive(Debug, Clone)]
pub struct Skills {
    pub acrobatics: Skill,
}

impl Skills {
    pub fn new(hero: Hero, attributes: DerivedAttributes) -> Self {
        let _ = hero;
        let _ = attributes;
        todo!("Implement Skills::new()")
    }
}
