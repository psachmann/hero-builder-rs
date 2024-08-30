use leptos::*;
use leptos_router::*;

use crate::components::AppState;

pub fn use_app_state() -> RwSignal<AppState> {
    expect_context::<RwSignal<AppState>>()
}

// FIXME: Check if lifetime is correct and if copy trait is needed
pub fn use_param<P: PartialEq + Params, R: PartialEq + Default>(
    params: Option<Memo<Result<P, ParamsError>>>,
    select: impl FnOnce(&P) -> R + Clone + Copy + 'static,
) -> Memo<R> {
    let params = match params {
        Some(params) => params,
        None => use_params::<P>(),
    };

    create_memo(move |_| params.with(move |params| params.as_ref().map(select).unwrap_or_default()))
}
