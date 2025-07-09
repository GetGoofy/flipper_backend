use std::collections::HashMap;

use axum::{http, Json};
use reqwest;
use serde;

use crate::variables::use_variable_TSM_API_KEY;




pub async fn reagents_get() -> Json<Vec<TsmPricingDataResponse>> {
    let http_client = reqwest::Client::new();


    let tsm_auth_body = TsmAuthBody {
        client_id: "c260f00d-1071-409a-992f-dda2e5498536".to_string(),
        grant_type: "api_token".to_string(),
        scope: "app:realm-api app:pricing-api".to_string(),
        token: use_variable_TSM_API_KEY(),
    };

    let resp_tsm_auth_body: TsmAuthResponse = http_client
        .post("https://auth.tradeskillmaster.com/oauth2/token")
        .json(&tsm_auth_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let access_token = resp_tsm_auth_body.access_token;

    let resp_tsm_region_pricing_data: Vec<TsmPricingDataResponse> = http_client
        .get("https://pricing-api.tradeskillmaster.com/region/1")
        .bearer_auth(&access_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();


    Json(resp_tsm_region_pricing_data)
}

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)] // this is needed if I print the entire JSON and not the specific fields, since its considered a side-effect and not real usage
struct UsersEndpointJSONBody {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

#[derive(Debug, serde::Serialize)]
struct TsmAuthBody {
    client_id: String,
    grant_type: String,
    scope: String,
    token: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TsmAuthResponse {
    access_token: String
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TsmPricingDataResponse {
    #[serde(rename = "regionId")]
    region_id: f64,
    #[serde(rename = "itemId")]
    item_id: Option<f64>,
    quantity: f64,
    #[serde(rename = "avgSalePrice")]
    avg_sale_price: f64,
    #[serde(rename = "soldPerDay")]
    sold_per_day: f64,
}
