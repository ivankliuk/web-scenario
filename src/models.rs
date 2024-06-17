use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) enum HttpMethod {
    GET,
    POST,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Step {
    pub method: HttpMethod,
    pub url: String,
    pub body: Option<String>,
}
