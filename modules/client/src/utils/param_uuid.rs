use leptos_router::{IntoParam, ParamsError};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ParamUuid(Uuid);

impl From<ParamUuid> for Uuid {
    fn from(param: ParamUuid) -> Self {
        param.0
    }
}

impl IntoParam for ParamUuid {
    fn into_param(value: Option<&str>, name: &str) -> Result<Self, ParamsError> {
        match value {
            Some(value) => match Uuid::parse_str(value) {
                Ok(uuid) => Ok(Self(uuid)),
                Err(err) => Err(ParamsError::Params(Arc::new(err))),
            },
            None => Err(ParamsError::MissingParam(name.to_string())),
        }
    }
}
