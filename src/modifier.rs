use std::str::FromStr;

use anyhow::Ok;
use http::uri::PathAndQuery;
use hyper::{Body, Request, Uri};
use typed_builder::TypedBuilder;

use crate::action::GhostAction;
use crate::config::GhostConfig;
use crate::http::QueryString;
use crate::keywords::*;
use crate::logging;

#[derive(TypedBuilder, Clone)]
pub struct GhostModifier {
    #[builder(default = false)]
    zero_dl: bool,
    #[builder(default = false)]
    intact_left: bool,
}

impl GhostModifier {
    pub fn new(config: &GhostConfig) -> Self {
        Self {
            zero_dl: config.zero_dl,
            intact_left: config.intact_left,
        }
    }

    pub fn execute(&self, req: &mut Request<Body>) -> anyhow::Result<GhostAction> {
        // Deserialize
        let mut uri_parts = req.uri().clone().into_parts();
        let Some(path_and_query) = &uri_parts.path_and_query else {
          return Ok(GhostAction::Continue);
        };
        let (path, Some(query)) = (path_and_query.path(), path_and_query.query()) else {
          return Ok(GhostAction::Continue);
        };

        if !path.ends_with("announce") {
            log::info!("Request is not an announce, skipping...");
            return Ok(GhostAction::Continue);
        }

        // Modify
        let mut query = QueryString::parse(query);

        let Some(info_hash) =  query.get(INFO_HASH) else {
            log::info!("Request is missing info hash, skipping...");
            return Ok(GhostAction::Continue);
        };
        let event = query.get(EVENT).unwrap_or("unknown event");

        log::info!(
            "Modifiying request for hash ({}): {}",
            event,
            logging::pretty_hash(info_hash)
        );

        if let Some(dl) = query.get_value::<u64>(DOWNLOADED) {
            if self.zero_dl {
                log::info!("Set downloaded to zero (from {})", dl);
                query.set(DOWNLOADED, "0");
            }

            if let Some(left) = query.get_value::<u64>(LEFT) {
                if self.intact_left {
                    let new_left = left + dl;
                    log::info!("Set left to {} (from {})", new_left, left);
                    query.set_value(LEFT, &new_left);
                }
            }
        }

        // Serialize
        let new_path_and_query = format!("{}?{}", path, query);
        uri_parts.path_and_query = Some(PathAndQuery::from_str(&new_path_and_query)?);
        *req.uri_mut() = Uri::from_parts(uri_parts)?;

        Ok(GhostAction::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializable() {
        let modifier = GhostModifier::builder().build();
        let mut req = Request::get("http://example.com:8080/cksbkjfbesjkfbsekjbfjkesbf/announce?info_hash=%c3%9a%1a%0d%c3%af%c2%b3%5dc%c2%a2%18%c3%bc%7e%c2%b0%c3%bc%c2%8dlld%c2%a3%c3%ad%2d&peer_id=-DE13F0-fksdjfbskf&port=52375&uploaded=0&downloaded=61931723&left=1663451457&corrupt=0&key=599CC07&event=stopped&numwant=0&compact=1&no_peer_id=1&supportcrypto=1&redundant=0").body(Body::empty()).unwrap();
        modifier.execute(&mut req).unwrap();
        assert_eq!("/cksbkjfbesjkfbsekjbfjkesbf/announce?info_hash=%c3%9a%1a%0d%c3%af%c2%b3%5dc%c2%a2%18%c3%bc%7e%c2%b0%c3%bc%c2%8dlld%c2%a3%c3%ad%2d&peer_id=-DE13F0-fksdjfbskf&port=52375&uploaded=0&downloaded=61931723&left=1663451457&corrupt=0&key=599CC07&event=stopped&numwant=0&compact=1&no_peer_id=1&supportcrypto=1&redundant=0", req.uri().path_and_query().unwrap().to_string())
    }

    #[test]
    fn test_zero_download() {
        let modifier = GhostModifier::builder().zero_dl(true).build();
        let mut req = Request::get("http://example.com:8080/cksbkjfbesjkfbsekjbfjkesbf/announce?info_hash=%c3%9a%1a%0d%c3%af%c2%b3%5dc%c2%a2%18%c3%bc%7e%c2%b0%c3%bc%c2%8dlld%c2%a3%c3%ad%2d&peer_id=-DE13F0-fksdjfbskf&port=52375&uploaded=0&downloaded=61931723&left=1663451457&corrupt=0&key=599CC07&event=stopped&numwant=0&compact=1&no_peer_id=1&supportcrypto=1&redundant=0").body(Body::empty()).unwrap();
        modifier.execute(&mut req).unwrap();
        assert_eq!("/cksbkjfbesjkfbsekjbfjkesbf/announce?info_hash=%c3%9a%1a%0d%c3%af%c2%b3%5dc%c2%a2%18%c3%bc%7e%c2%b0%c3%bc%c2%8dlld%c2%a3%c3%ad%2d&peer_id=-DE13F0-fksdjfbskf&port=52375&uploaded=0&downloaded=0&left=1663451457&corrupt=0&key=599CC07&event=stopped&numwant=0&compact=1&no_peer_id=1&supportcrypto=1&redundant=0", req.uri().path_and_query().unwrap().to_string())
    }

    #[test]
    fn test_left_intact() {
        let modifier = GhostModifier::builder().intact_left(true).build();
        let mut req = Request::get("http://example.com:8080/cksbkjfbesjkfbsekjbfjkesbf/announce?info_hash=%c3%9a%1a%0d%c3%af%c2%b3%5dc%c2%a2%18%c3%bc%7e%c2%b0%c3%bc%c2%8dlld%c2%a3%c3%ad%2d&peer_id=-DE13F0-fksdjfbskf&port=52375&uploaded=0&downloaded=61931723&left=1663451457&corrupt=0&key=599CC07&event=stopped&numwant=0&compact=1&no_peer_id=1&supportcrypto=1&redundant=0").body(Body::empty()).unwrap();
        modifier.execute(&mut req).unwrap();
        assert_eq!("/cksbkjfbesjkfbsekjbfjkesbf/announce?info_hash=%c3%9a%1a%0d%c3%af%c2%b3%5dc%c2%a2%18%c3%bc%7e%c2%b0%c3%bc%c2%8dlld%c2%a3%c3%ad%2d&peer_id=-DE13F0-fksdjfbskf&port=52375&uploaded=0&downloaded=61931723&left=1725383180&corrupt=0&key=599CC07&event=stopped&numwant=0&compact=1&no_peer_id=1&supportcrypto=1&redundant=0", req.uri().path_and_query().unwrap().to_string())
    }
}
