use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub hero_rank: RwSignal<i32>,
    pub size_class: RwSignal<i32>,
    pub available_xp: RwSignal<i32>,
    pub spent_xp: RwSignal<i32>,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            name: String::default().into(),
            hero_rank: 1.into(),
            size_class: 0.into(),
            available_xp: 0.into(),
            spent_xp: 0.into(),
        }
    }
}

impl Hero {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: RwSignal::new(name),
            ..Default::default()
        }
    }
}
