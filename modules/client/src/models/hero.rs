use leptos::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Hero {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub size_class: RwSignal<i32>,
}

impl Hero {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: RwSignal::new(name),
            size_class: RwSignal::new(0),
        }
    }
}
