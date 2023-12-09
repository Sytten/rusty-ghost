use hyper::{Body, Response};

#[allow(dead_code)]
pub enum GhostAction {
    Return(Response<Body>),
    Continue,
}
