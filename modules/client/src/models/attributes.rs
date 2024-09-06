use leptos::*;
use serde::Serialize;

/// Represents an attribute of a character.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Attribute {
    /// The current value of the attribute.
    pub value: Memo<i32>,
    /// The initial value of the attribute.
    pub initial: RwSignal<i32>,
    /// The trained value of the attribute.
    pub trained: RwSignal<i32>,
    /// The modifier value of the attribute.
    pub modifier: RwSignal<i32>,
}

impl Default for Attribute {
    fn default() -> Self {
        let initial = RwSignal::new(0);
        let trained = RwSignal::new(0);
        let modifier = RwSignal::new(0);
        let value = Memo::new(move |_| initial.get() + trained.get() + modifier.get());

        Self {
            value,
            initial,
            trained,
            modifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct Attributes {
    /// The charisma of the hero. German: Ausstrahlung
    pub charisma: Attribute,
    /// The agility of the hero. German: Beweglichkeit
    pub agility: Attribute,
    /// The intuition of the hero. German: Intuition
    pub intuition: Attribute,
    /// The constitution of the hero. German: Konstitution
    pub constitution: Attribute,
    /// The mysticism of the hero. German: Mystik
    pub mysticism: Attribute,
    /// The strength of the hero. German: Stärke
    pub strength: Attribute,
    /// The intelligence of the hero. German: Verstand
    pub intelligence: Attribute,
    /// The willpower of the hero. German: Willenskraft
    pub willpower: Attribute,
}

impl IntoIterator for Attributes {
    type Item = (i32, Attribute);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (0, self.charisma),
            (1, self.agility),
            (2, self.intuition),
            (3, self.constitution),
            (4, self.mysticism),
            (5, self.strength),
            (6, self.intelligence),
            (7, self.willpower),
        ]
        .into_iter()
    }
}
