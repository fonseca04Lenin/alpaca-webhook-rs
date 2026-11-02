use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse, 
    routing::{get, post},
    Router,
};

use dotenvy::dotenv;
use reqwest::CLient;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct AlpacaOrder{
    action: String,
    symbol: String,
    quantity: u32,
    oder_type: String
    #[serde(default)]
    price: Option<f64>,
    secret: String
}

#[derive(Serialize)]
struct WebhookResponse {
    status: String,
    message: String,
}
#[derive(Debug, Serialize)]
struct AlpacaOrder {
    Symbol: String,
    qty: String,
    side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<String>,

}
Async fn place_alapaca_order(signal: &TradeSignal) -> Result<String, String> {
    let alpaca_key = std::env::var("ALPACA_KEY").map_err(|_| "ALPACA_KEY not set".to_string())?;
    let alpaca_secret=
    std::env::var("ALPACA_SECRET").map_err(|_| "ALPACA_SECRET not set".to_string())?;
    let base_url =std::env:var (ALPACA_BASE_URL).unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string());

    let order = AlpacaOrder {
        Symbol: signal.Symbol.clone(),
        qty: signal.qty.to_string(),
        side: signal.action.clone(), 
        order_type:signal.order_type.clone(),
        time_in_force: "day".to_string(),
        limit_price: signal.price.map(|p| p.to_string()),
    };
    let  client = Client ::new();
    let url = format!("{}/v2/orders", base_url)

    let response = client 
    .post(&url)
    .header("APCA-API-KEY-ID", alpaca_key)
    .header("APCA-API-SECRET-KEY", alpaca_secret)
    .json(&order)
    .send()
    .await
    .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    
    if status.is_success() {
        Ok(body)
    } else {
        error!("Alpaca API errorr {}: {}", status, body)
        Err(format!("Alpaca API error: {} - {}", status, body)
    }
}

async fn handle_webhook(Json(signal): Json<TradeSignal>) -> impl IntoResponse {
    info!("Received webhook: {:?}", signal);
    let webhook_secret = std::env::var("WEBHOOK_SECRET").unwrap_or_else(|_| "deafult".to_string())
    if signal.secret !=webhook_secret{
        warn!("Invalid secret in webhook request");
        return(
            StatusCode:: FORBIDDEN,
            Json(WebhookResponse {
                status: "error".to_string(), 
                message: "Invalid secret".to_string(),

            }),
        );
    }

    if signal.action != "buy" && signal.action != "sell" {
        warn!("Invalid action: {}", signal.action);
        return(
            StatusCode::BAD_REQUEST,
            Json(WebhookResponse{
                status: "error".to_string(),
                message: "Action required must buy or sell".to_string(),
            }),
        );
    }
    match place_alpaca_order(&signal).await {
        Ok(order_response)=>{
            info!("Order placed sucessfully: {}", order_response);
            ()
        }
    }
}
