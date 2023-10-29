use http::StatusCode;
use hyper::{Body, Request};
use simple_proxy::proxy::error::MiddlewareError;
use simple_proxy::proxy::middleware::{Middleware, MiddlewareResult};
use simple_proxy::proxy::service::{ServiceContext, State};

use crate::config::GhostConfig;
use crate::modifier::GhostModifier;

pub struct GhostMiddleware {
    modifier: GhostModifier,
}

impl GhostMiddleware {
    pub fn new(config: &GhostConfig) -> Self {
        Self {
            modifier: GhostModifier::new(config),
        }
    }
}

impl Middleware for GhostMiddleware {
    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        _ctx: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        let action = self.modifier.execute(req).map_err(|err| {
            log::error!("Error while executing modifier: {err:?}");
            MiddlewareError::new(
                "Failed to execute modifier".to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;
        Ok(action.into())
    }

    fn name() -> String
    where
        Self: Sized,
    {
        "Ghost".to_string()
    }
}
