use actix_web::body::BoxBody;

pub struct APIList<DATA> {
    pub total_count: usize,
    pub list: std::vec::Vec<DATA>,
}

impl<DATA> APIList<DATA> {
    pub fn new(list: std::vec::Vec<DATA>, total_count: i64) -> APIList<DATA> {
        Self {
            total_count: total_count as usize,
            list,
        }
    }

    pub fn ok(list: std::vec::Vec<DATA>, total_count: i64) -> actix_web::Result<APIList<DATA>> {
        Ok(Self::new(list, total_count))
    }
}

impl<DATA: serde::Serialize> actix_web::Responder for APIList<DATA> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body: String = serde_json::ser::to_string(&self.list).unwrap();
        actix_web::HttpResponseBuilder::new(actix_web::http::StatusCode::OK)
            .append_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .append_header(("X-Total-Count", self.total_count.to_string()))
            .body(body)
    }
}
