use std::env;

use lazy_static::lazy_static;
use reqwest::Client;

#[allow(non_snake_case)]
pub fn use_variable_TSM_API_KEY() -> String {
    let TSM_API_KEY: String = env::var("TSM_API_KEY").expect("TSM_API_KEY must be set").to_string();
    TSM_API_KEY
}

lazy_static! {
    pub static ref HTTP_CLIENT: Client = reqwest::Client::new();

    pub static ref TSM_AUTH_BODY: TsmAuthBody = TsmAuthBody {
        client_id: "c260f00d-1071-409a-992f-dda2e5498536".to_string(),
        grant_type: "api_token".to_string(),
        scope: "app:realm-api app:pricing-api".to_string(),
        token: use_variable_TSM_API_KEY(),
    };
}

#[derive(Debug, serde::Serialize)]
pub struct TsmAuthBody {
    client_id: String,
    grant_type: String,
    scope: String,
    token: String,
}