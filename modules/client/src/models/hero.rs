use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Reflect, PartialEq)]
pub struct Hero {
    pub id: Uuid,
    pub name: String,
    pub hero_rank: i32,
    pub size_class: i32,
    pub available_xp: i32,
    pub spent_xp: i32,
}
