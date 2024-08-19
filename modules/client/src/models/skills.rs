use leptos::*;

/// A skill of a hero.
/// 0: Points invested in the skill.
/// 2: The temporary points that can be added to the skill.
/// 3: The memoized value of the skill.
pub type Skill = (RwSignal<i32>, RwSignal<i32>, Memo<i32>);

#[derive(Debug, Clone)]
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
