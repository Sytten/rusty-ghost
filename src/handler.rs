use async_trait::async_trait;
use http::StatusCode;
use http::{Request, Response};
use hudsucker::{HttpContext, HttpHandler, RequestOrResponse};
use hyper::Body;

use crate::action::GhostAction;
use crate::config::GhostConfig;
use crate::modifier::GhostModifier;

#[derive(Clone)]
pub struct GhostHandler {
    modifier: GhostModifier,
}

impl GhostHandler {
    pub fn new(config: &GhostConfig) -> Self {
        Self {
            modifier: GhostModifier::new(config),
        }
    }
}

#[async_trait]
impl HttpHandler for GhostHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        mut req: Request<Body>,
    ) -> RequestOrResponse {
        match self.modifier.execute(&mut req) {
            Ok(action) => match action {
                GhostAction::Return(res) => res.into(),
                GhostAction::Continue => req.into(),
            },
            Err(err) => {
                log::error!("Error while executing modifier: {err:?}");
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .expect("Failed to build response")
                    .into()
            }
        }
    }

    async fn handle_error(&mut self, _ctx: &HttpContext, err: hyper::Error) -> Response<Body> {
        log::error!("Failed to forward request: {:?}", err);
        Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .body(Body::empty())
            .expect("Failed to build response")
    }
}
