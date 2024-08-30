use leptos::*;

#[component]
pub fn sidebar() -> impl IntoView {
    view! {
        <div class="text-white">
            <h1>{"Sidebar"}</h1>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarState {
    pub is_open: bool,
    pub width: f32,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            is_open: false,
            width: 200.0,
        }
    }
}
