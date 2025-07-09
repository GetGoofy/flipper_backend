use axum::{Json};
use serde;

use crate::variables::{ HTTP_CLIENT, TSM_AUTH_BODY};

#[axum::debug_handler]
pub async fn reagents_get() -> Json<Vec<TsmPricingDataResponse>> {

    let resp_tsm_auth_body: TsmAuthResponse = HTTP_CLIENT
        .post("https://auth.tradeskillmaster.com/oauth2/token")
        .json(&*TSM_AUTH_BODY)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let resp_tsm_region_pricing_data: Vec<TsmPricingDataResponse> = HTTP_CLIENT
        .get("https://pricing-api.tradeskillmaster.com/region/1")
        .bearer_auth(&resp_tsm_auth_body.access_token)
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
