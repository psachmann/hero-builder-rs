use leptos::*;
use serde::Serialize;
use uuid::Uuid;

use super::{Attributes, DerivedAttributes, Skills};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Hero {
    pub id: Uuid,
    pub name: String,
    pub hero_rank: i32,
    pub size_class: RwSignal<i32>,
    pub available_xp: i32,
    pub spent_xp: i32,
    pub race_modifier: RwSignal<i32>,
    pub attributes: Attributes,
    pub derived_attributes: DerivedAttributes,
    pub skills: Skills,
}

impl Default for Hero {
    fn default() -> Self {
        let size_class = RwSignal::new(0);
        let race_modifier = RwSignal::new(0);
        let attributes = Attributes::default();
        let derived_attributes = DerivedAttributes::new(
            size_class.clone(),
            race_modifier.clone(),
            attributes.clone(),
        );
        let skills = Skills::new(attributes.clone());

        Self {
            id: Uuid::default(),
            name: String::default(),
            hero_rank: 1,
            size_class,
            available_xp: 0,
            spent_xp: 0,
            race_modifier,
            attributes,
            derived_attributes,
            skills  ,
        }
    }
}
