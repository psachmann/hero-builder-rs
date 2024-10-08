use leptos::*;
use leptos_router::*;

use crate::{components::AppState, models::Hero};

use super::ParamUuid;

pub fn use_app_state() -> RwSignal<AppState> {
    expect_context::<RwSignal<AppState>>()
}

pub fn use_param<P, R>(
    params: Option<Memo<Result<P, ParamsError>>>,
    select: impl FnOnce(&P) -> R + Copy + 'static,
) -> Memo<R>
where
    P: PartialEq + Params,
    R: PartialEq + Default,
{
    let params = match params {
        Some(params) => params,
        None => use_params::<P>(),
    };

    create_memo(move |_| params.with(move |params| params.as_ref().map(select).unwrap_or_default()))
}

pub fn use_hero_with_id(id: Memo<ParamUuid>) -> (Signal<Hero>, SignalSetter<Hero>) {
    let state = use_app_state();

    create_slice(
        state,
        move |state| {
            state
                .heros
                .clone()
                .into_iter()
                .find(|hero| hero.id == id.get().into())
                .unwrap_or_default()
        },
        |state, hero: Hero| {
            state.heros = state
                .heros
                .clone()
                .into_iter()
                .map(|h| if h.id == hero.id { hero.clone() } else { h })
                .collect();
        },
    )
}
