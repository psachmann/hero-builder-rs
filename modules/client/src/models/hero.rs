use leptos::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Hero {
    pub id: Uuid,
    pub name: RwSignal<String>,
}

impl Hero {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: RwSignal::new(name),
        }
    }
}
