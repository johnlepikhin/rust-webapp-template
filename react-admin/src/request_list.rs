use utoipa::IntoParams;

fn default_end() -> u64 {
    10
}

/// Pagination request data
#[derive(serde::Deserialize, IntoParams)]
pub struct PaginatedRequest {
    /// Starting record in paginated output
    #[serde(rename = "_start", default)]
    pub start: u64,
    /// Ending record in paginated output
    #[serde(rename = "_end", default = "default_end")]
    pub end: u64,
}

pub struct ProcessedPaginatedRequest {
    pub offset: i64,
    pub limit: i64,
}

impl actix_web::FromRequest for ProcessedPaginatedRequest {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<actix_web::Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let query = req.query_string();
        let query = match serde_urlencoded::from_str::<PaginatedRequest>(query) {
            Ok(v) => v,
            Err(err) => {
                return futures::future::err(actix_web::error::ErrorBadRequest(err.to_string()))
            }
        };

        if query.start > query.end {
            return futures::future::err(actix_web::error::ErrorBadRequest(
                "_start query param must be less than _end",
            ));
        }

        futures::future::ok(ProcessedPaginatedRequest {
            offset: query.start as i64,
            limit: query.end as i64 - query.start as i64,
        })
    }
}
