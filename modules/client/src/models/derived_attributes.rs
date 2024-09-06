use leptos::*;
use serde::Serialize;

use super::Attributes;

/// A derived attribute consists of a value and a modifier.
/// The value is stored as a `Memo` type, which allows for memoization of the attribute's value.
/// The modifier is stored as a `RwSignal` type, which allows for reading and writing the attribute's modifier.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DerivedAttribute {
    /// The value of the attribute.
    pub value: Memo<i32>,
    /// The modifier of the attribute.
    pub modifier: RwSignal<i32>,
}

impl DerivedAttribute {
    /// Creates a new derived attribute with the given formula.
    pub fn new(value: Memo<i32>, modifier: RwSignal<i32>) -> Self {
        Self { value, modifier }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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

impl DerivedAttributes {
    pub fn new(
        size_class: RwSignal<i32>,
        race_modifier: RwSignal<i32>,
        attributes: Attributes,
    ) -> Self {
        let size_class_modifier = RwSignal::new(0);
        let size_class = Memo::new(move |_| size_class.get() + size_class_modifier.get());

        let speed_modifier = RwSignal::new(0);
        let speed = Memo::new(move |_| {
            size_class.get() + attributes.agility.value.get() + speed_modifier.get()
        });

        let initiative_modifier = RwSignal::new(0);
        let initiative =
            Memo::new(move |_| 10 - attributes.intuition.value.get() + initiative_modifier.get());

        let hit_points_modifier = RwSignal::new(0);
        let hit_points = Memo::new(move |_| {
            size_class.get() + attributes.constitution.value.get() + hit_points_modifier.get()
        });

        let focus_modifier = RwSignal::new(0);
        let focus = Memo::new(move |_| {
            2 * (attributes.mysticism.value.get()
                + attributes.willpower.value.get()
                + focus_modifier.get())
        });

        let defense_modifier = RwSignal::new(0);
        let defense = Memo::new(move |_| {
            12 + attributes.agility.value.get()
                + attributes.strength.value.get()
                + defense_modifier.get()
                + race_modifier.get()
        });

        let mental_resistance_modifier = RwSignal::new(0);
        let mental_resistance = Memo::new(move |_| {
            12 + attributes.intelligence.value.get()
                + attributes.willpower.value.get()
                + mental_resistance_modifier.get()
        });

        let physical_resistance_modifier = RwSignal::new(0);
        let physical_resistance = Memo::new(move |_| {
            12 + attributes.constitution.value.get()
                + attributes.willpower.value.get()
                + physical_resistance_modifier.get()
        });

        Self {
            size_class: DerivedAttribute::new(size_class, size_class_modifier),
            speed: DerivedAttribute::new(speed, speed_modifier),
            initiative: DerivedAttribute::new(initiative, initiative_modifier),
            hit_points: DerivedAttribute::new(hit_points, hit_points_modifier),
            focus: DerivedAttribute::new(focus, focus_modifier),
            defense: DerivedAttribute::new(defense, defense_modifier),
            mental_resistance: DerivedAttribute::new(mental_resistance, mental_resistance_modifier),
            physical_resistance: DerivedAttribute::new(
                physical_resistance,
                physical_resistance_modifier,
            ),
        }
    }
}

impl IntoIterator for DerivedAttributes {
    type Item = (i32, DerivedAttribute);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (0, self.size_class),
            (1, self.speed),
            (2, self.initiative),
            (3, self.hit_points),
            (4, self.focus),
            (5, self.defense),
            (6, self.mental_resistance),
            (7, self.physical_resistance),
        ]
        .into_iter()
    }
}
