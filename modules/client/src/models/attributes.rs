use leptos::*;
use serde::{Deserialize, Serialize};

/// The attributes of a hero.
/// 0: The initial value of the attribute after character creation.
/// 1: The current value of invested points in the attribute.
/// 2: Temporary points that can be added to the attribute.
/// 3: The memoized value of the attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Attribute(pub RwSignal<i32>, pub RwSignal<i32>, pub RwSignal<i32>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

/// The derived attributes of a hero.
/// 0: The memoized value of the attribute.
/// 1: Temporary points that can be added to the attribute.
pub type DerivedAttribute = (Memo<i32>, RwSignal<i32>);

#[derive(Debug, Clone)]
pub struct DerivedAttributes {
    /// The size class of the hero calculated from race. German: Größenklasse
    pub size_class: DerivedAttribute,
    /// The speed of the hero calculated by size class + agility. German: Geschwindigkeit
    pub speed: DerivedAttribute,
    /// The initiative of the hero calculated by 10 - intuition. German: Initiative
    pub initiative: DerivedAttribute,
    /// The hit points of the hero calculated by size class + constitution. German: Lebenspunk
    pub hit_points: DerivedAttribute,
    /// The focus of the hero calculated by 2 x (mysticism + willpower). German: Fokus
    pub focus: DerivedAttribute,
    /// The defense of the hero calculated by 12 + agility + strength ± race. German: Verteidigung
    pub defense: DerivedAttribute,
    /// The mental resistance of the hero calculated by 12 + intelligence + willpower. German: Geistiger Widerstand
    pub mental_resistance: DerivedAttribute,
    /// The physical resistance of the hero calculated by 12 + constitution + willpower. German: Körperlicher Widerstand
    pub physical_resistance: DerivedAttribute,
}

/*
impl DerivedAttributes {
    pub fn new(hero: Hero, attributes: Attributes) -> Self {
        let size_class_modifier = RwSignal::new(0);
        let size_class = Memo::new(move |_| hero.size_class + size_class_modifier.get());

        let speed_modifier = RwSignal::new(0);
        let speed = Memo::new(move |_| {
            size_class.get() + attributes.agility.3.get() + speed_modifier.get()
        });

        let initiative_modifier = RwSignal::new(0);
        let initiative =
            Memo::new(move |_| 10 - attributes.intuition.3.get() + initiative_modifier.get());

        let hit_points_modifier = RwSignal::new(0);
        let hit_points = Memo::new(move |_| {
            size_class.get() + attributes.constitution.3.get() + hit_points_modifier.get()
        });

        let focus_modifier = RwSignal::new(0);
        let focus = Memo::new(move |_| {
            2 * (attributes.mysticism.3.get() + attributes.willpower.3.get() + focus_modifier.get())
        });

        let defense_modifier = RwSignal::new(0);
        let defense = Memo::new(move |_| {
            12 + attributes.agility.3.get() + attributes.strength.3.get() + defense_modifier.get()
        }); // TODO: add race modifier

        let mental_resistance_modifier = RwSignal::new(0);
        let mental_resistance = Memo::new(move |_| {
            12 + attributes.intelligence.3.get()
                + attributes.willpower.3.get()
                + mental_resistance_modifier.get()
        });
        let physical_resistance_modifier = RwSignal::new(0);
        let physical_resistance = Memo::new(move |_| {
            12 + attributes.constitution.3.get()
                + attributes.willpower.3.get()
                + physical_resistance_modifier.get()
        });

        Self {
            size_class: (size_class, size_class_modifier),
            speed: (speed, speed_modifier),
            initiative: (initiative, initiative_modifier),
            hit_points: (hit_points, hit_points_modifier),
            focus: (focus, focus_modifier),
            defense: (defense, defense_modifier),
            mental_resistance: (mental_resistance, mental_resistance_modifier),
            physical_resistance: (physical_resistance, physical_resistance_modifier),
        }
    }
}
*/
