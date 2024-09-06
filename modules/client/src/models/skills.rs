use leptos::*;
use serde::Serialize;

use super::Attributes;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Skill {
    pub value: Memo<i32>,
    pub trained: RwSignal<i32>,
    pub modifier: RwSignal<i32>,
}

impl Skill {
    pub fn new(attributes: Memo<i32>) -> Self {
        let trained = RwSignal::new(0);
        let modifier = RwSignal::new(0);
        let value = Memo::new(move |_| attributes.get() + trained.get() + modifier.get());

        Self {
            value,
            trained,
            modifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Skills {
    /// Represents the acrobatics skill (Akrobatik). Composing attributes: BEW, STÄ
    pub acrobatics: Skill,
    /// Represents the alchemy skill (Alchemie). Composing attributes: MYS, VER
    pub alchemy: Skill,
    /// Represents the leadership skill (Anführen). Composing attributes: AUS, WIL
    pub leadership: Skill,
    /// Represents the arcane knowledge skill (Arkane Kunde). Composing attributes: MYS, VER
    pub arcane_knowledge: Skill,
    /// Represents the athletics skill (Athletik). Composing attributes: BEW, STÄ
    pub athletics: Skill,
    /// Represents the performance skill (Darbietung). Composing attributes: AUS, WIL
    pub performance: Skill,
    /// Represents the diplomacy skill (Diplomatie). Composing attributes: AUS, VER
    pub diplomacy: Skill,
    /// Represents the fine craftsmanship skill (Edelhandwerk). Composing attributes: INT, VER
    pub fine_craftsmanship: Skill,
    /// Represents the empathy skill (Empathie). Composing attributes: INT, VER
    pub empathy: Skill,
    /// Represents the determination skill (Entschlossenheit). Composing attributes: AUS, WIL
    pub determination: Skill,
    /// Represents the dexterity skill (Fingerfertigkeit). Composing attributes: AUS, BEW
    pub dexterity: Skill,
    /// Represents the history & myths skill (Geschichte & Mythen). Composing attributes: MYS, VER
    pub history_myths: Skill,
    /// Represents the craftsmanship skill (Handwerk). Composing attributes: KON, VER
    pub craftsmanship: Skill,
    /// Represents the healing skill (Heilkunde). Composing attributes: INT, VER
    pub healing: Skill,
    /// Represents the stealth skill (Heimlichkeit). Composing attributes: BEW, INT
    pub stealth: Skill,
    /// Represents the hunting skill (Jagdkunst). Composing attributes: KON, VER
    pub hunting: Skill,
    /// Represents the geography skill (Länderkunde). Composing attributes: INT, VER
    pub geography: Skill,
    /// Represents the nature knowledge skill (Naturkunde). Composing attributes: INT, VER
    pub nature_knowledge: Skill,
    /// Represents the eloquence skill (Redegewandtheit). Composing attributes: AUS, WIL
    pub eloquence: Skill,
    /// Represents the locks & traps skill (Schlösser & Fallen). Composing attributes: INT, BEW
    pub locks_traps: Skill,
    /// Represents the swimming skill (Schwimmen). Composing attributes: STÄ, KON
    pub swimming: Skill,
    /// Represents the seafaring skill (Seefahrt). Composing attributes: BEW, KON
    pub seafaring: Skill,
    /// Represents the street knowledge skill (Straßenkunde). Composing attributes: AUS, INT
    pub street_knowledge: Skill,
    /// Represents the animal handling skill (Tierführung). Composing attributes: AUS, BEW
    pub animal_handling: Skill,
    /// Represents the survival skill (Überleben). Composing attributes: INT, KON
    pub survival: Skill,
    /// Represents the perception skill (Wahrnehmung). Composing attributes: INT, WIL
    pub perception: Skill,
    /// Represents the toughness skill (Zähigkeit). Composing attributes: KON, WIL
    pub toughness: Skill,
}

impl Skills {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            acrobatics: Skill::new(Memo::new(move |_| {
                attributes.agility.value.get() + attributes.strength.value.get()
            })),
            alchemy: Skill::new(Memo::new(move |_| {
                attributes.mysticism.value.get() + attributes.intelligence.value.get()
            })),
            leadership: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.willpower.value.get()
            })),
            arcane_knowledge: Skill::new(Memo::new(move |_| {
                attributes.mysticism.value.get() + attributes.intelligence.value.get()
            })),
            athletics: Skill::new(Memo::new(move |_| {
                attributes.agility.value.get() + attributes.strength.value.get()
            })),
            performance: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.willpower.value.get()
            })),
            diplomacy: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.intelligence.value.get()
            })),
            fine_craftsmanship: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.willpower.value.get()
            })),
            empathy: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.willpower.value.get()
            })),
            determination: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.willpower.value.get()
            })),
            dexterity: Skill::new(Memo::new(move |_| {
                attributes.agility.value.get() + attributes.intelligence.value.get()
            })),
            history_myths: Skill::new(Memo::new(move |_| {
                attributes.mysticism.value.get() + attributes.intelligence.value.get()
            })),
            craftsmanship: Skill::new(Memo::new(move |_| {
                attributes.constitution.value.get() + attributes.willpower.value.get()
            })),
            healing: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.willpower.value.get()
            })),
            stealth: Skill::new(Memo::new(move |_| {
                attributes.agility.value.get() + attributes.intelligence.value.get()
            })),
            hunting: Skill::new(Memo::new(move |_| {
                attributes.constitution.value.get() + attributes.willpower.value.get()
            })),
            geography: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.willpower.value.get()
            })),
            nature_knowledge: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.willpower.value.get()
            })),
            eloquence: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.willpower.value.get()
            })),
            locks_traps: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.agility.value.get()
            })),
            swimming: Skill::new(Memo::new(move |_| {
                attributes.strength.value.get() + attributes.constitution.value.get()
            })),
            seafaring: Skill::new(Memo::new(move |_| {
                attributes.agility.value.get() + attributes.constitution.value.get()
            })),
            street_knowledge: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.intelligence.value.get()
            })),
            animal_handling: Skill::new(Memo::new(move |_| {
                attributes.charisma.value.get() + attributes.agility.value.get()
            })),
            survival: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.constitution.value.get()
            })),
            perception: Skill::new(Memo::new(move |_| {
                attributes.intelligence.value.get() + attributes.willpower.value.get()
            })),
            toughness: Skill::new(Memo::new(move |_| {
                attributes.constitution.value.get() + attributes.willpower.value.get()
            })),
        }
    }
}

impl IntoIterator for Skills {
    type Item = (String, Skill);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    // TODO: This is a bit ugly. Maybe we can use a macro to generate this?
    // TODO: Make the String -> i32 for range support in leptos i18n
    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("acrobatics".to_string(), self.acrobatics),
            ("alchemy".to_string(), self.alchemy),
            ("leadership".to_string(), self.leadership),
            ("arcane_knowledge".to_string(), self.arcane_knowledge),
            ("athletics".to_string(), self.athletics),
            ("performance".to_string(), self.performance),
            ("diplomacy".to_string(), self.diplomacy),
            ("fine_craftsmanship".to_string(), self.fine_craftsmanship),
            ("empathy".to_string(), self.empathy),
            ("determination".to_string(), self.determination),
            ("dexterity".to_string(), self.dexterity),
            ("history_myths".to_string(), self.history_myths),
            ("craftsmanship".to_string(), self.craftsmanship),
            ("healing".to_string(), self.healing),
            ("stealth".to_string(), self.stealth),
            ("hunting".to_string(), self.hunting),
            ("geography".to_string(), self.geography),
            ("nature_knowledge".to_string(), self.nature_knowledge),
            ("eloquence".to_string(), self.eloquence),
            ("locks_traps".to_string(), self.locks_traps),
            ("swimming".to_string(), self.swimming),
            ("seafaring".to_string(), self.seafaring),
            ("street_knowledge".to_string(), self.street_knowledge),
            ("animal_handling".to_string(), self.animal_handling),
            ("survival".to_string(), self.survival),
            ("perception".to_string(), self.perception),
            ("toughness".to_string(), self.toughness),
        ]
        .into_iter()
    }
}
