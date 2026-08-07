pub struct Response {
    pub exit_code: i32,
    pub body: String,
    pub json: Option<String>,
}

impl Response {
    pub fn document(exit_code: i32, body: String, json: String) -> Self {
        Self {
            exit_code,
            body,
            json: Some(json),
        }
    }
}

impl From<(i32, String)> for Response {
    fn from((exit_code, body): (i32, String)) -> Self {
        Self {
            exit_code,
            body,
            json: None,
        }
    }
}
