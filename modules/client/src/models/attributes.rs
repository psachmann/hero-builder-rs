use leptos::*;

#[derive(Debug, Clone)]
pub struct Attribute {
    /// The initial value of the attribute after character creation.
    pub initial: RwSignal<i32>,
    /// The current value of invested points in the attribute.
    pub current: RwSignal<i32>,
    /// Temporary points that can be added to the attribute.
    pub modifier: RwSignal<i32>,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            initial: RwSignal::new(0),
            current: RwSignal::new(0),
            modifier: RwSignal::new(0),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DerivedAttributes {}
