use hyper::{Body, Response};
use simple_proxy::proxy::middleware::MiddlewareResult;

pub enum GhostAction {
    Return(Response<Body>),
    Continue,
}

impl From<GhostAction> for MiddlewareResult {
    fn from(value: GhostAction) -> Self {
        match value {
            GhostAction::Return(res) => MiddlewareResult::RespondWith(res),
            GhostAction::Continue => MiddlewareResult::Next,
        }
    }
}
