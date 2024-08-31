use leptos::*;
use leptos_router::*;

use crate::utils::ParamUuid;

#[derive(PartialEq, Params)]
pub struct EditorParams {
    /// The UUID of the hero to edit.
    pub id: ParamUuid,
}
