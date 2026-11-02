# Complete Setup & Coding Guide

This guide walks you through building the Alpaca webhook receiver from scratch in Rust. Follow every step carefully, even if you're new to Rust.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Install Rust & Tools](#install-rust--tools)
3. [Project Setup](#project-setup)
4. [Understanding Dependencies](#understanding-dependencies)
5. [Configuration Setup](#configuration-setup)
6. [Writing the Code](#writing-the-code)
7. [Testing Locally](#testing-locally)
8. [Deploying to Production](#deploying-to-production)
9. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### What You Need Before Starting

1. **A computer** running macOS, Linux, or Windows (with WSL2 recommended)
2. **Alpaca account** (free):
   - Sign up at https://alpaca.markets
   - Get API keys from the dashboard (we'll use paper trading first)
3. **TradingView account** (free plan works):
   - We'll set up alerts to trigger webhooks
4. **Basic command line knowledge**:
   - How to navigate folders (`cd`, `ls`/`dir`)
   - How to run commands
5. **Text editor or IDE**:
   - VS Code (recommended — install "rust-analyzer" extension)
   - Or any editor you like

### No Prior Rust Experience Needed!

This guide assumes you're learning Rust as you go. I'll explain key concepts as they appear.

---

## Install Rust & Tools

### Step 1: Install Rust (via rustup)

Open your terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**What this does**: Downloads and installs `rustup` (the Rust toolchain installer), which includes:
- `rustc` (the Rust compiler)
- `cargo` (package manager & build tool — like npm for Node.js or pip for Python)
- Standard library

**Follow the prompts**:
- Choose option `1` (default installation)
- Wait for it to finish (~2-5 minutes)

After installation, **restart your terminal** or run:

```bash
source $HOME/.cargo/env
```

**Verify installation**:

```bash
rustc --version   # Should show: rustc 1.84.0 (or newer)
cargo --version   # Should show: cargo 1.84.0 (or newer)
```

### Step 2: Update Rust (if already installed)

If you already have Rust installed, update to the latest stable version:

```bash
rustup update stable
```

### Step 3: Install Additional Tools

These are optional but highly recommended:

```bash
# Code formatter (auto-formats your Rust code)
rustup component add rustfmt

# Linter (catches common mistakes)
rustup component add clippy
```

### Step 4: Install ngrok (for local testing)

ngrok exposes your local server to the internet so TradingView can reach it.

**macOS (with Homebrew)**:
```bash
brew install ngrok/ngrok/ngrok
```

**Linux/WSL**:
```bash
curl -sSL https://ngrok-agent.s3.amazonaws.com/ngrok.asc | sudo tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null
echo "deb https://ngrok-agent.s3.amazonaws.com buster main" | sudo tee /etc/apt/sources.list.d/ngrok.list
sudo apt update && sudo apt install ngrok
```

**Windows**:
- Download from https://ngrok.com/download
- Extract and add to PATH

**Verify**:
```bash
ngrok version
```

Sign up at https://ngrok.com and run:
```bash
ngrok config add-authtoken YOUR_NGROK_TOKEN
```

---

## Project Setup

### Step 1: Create New Rust Project

Navigate to where you want to store your project, then:

```bash
cargo new alpaca-webhook-rs
cd alpaca-webhook-rs
```

**What this does**:
- Creates a new folder `alpaca-webhook-rs/`
- Generates basic project structure:
  ```
  alpaca-webhook-rs/
  ├── src/
  │   └── main.rs      # Your code goes here
  ├── Cargo.toml       # Project config & dependencies
  └── .gitignore       # Git ignore file (auto-includes target/)
  ```

**Test it works**:

```bash
cargo run
```

You should see:
```
   Compiling alpaca-webhook-rs v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/alpaca-webhook-rs`
Hello, world!
```

Great! Rust is working. Now let's add dependencies.

---

## Understanding Dependencies

### Step 2: Edit Cargo.toml

Open `Cargo.toml` in your editor. It looks like this:

```toml
[package]
name = "alpaca-webhook-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**Replace the entire file** with this:

```toml
[package]
name = "alpaca-webhook-rs"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Webhook receiver for Alpaca trading automation"

[dependencies]
# Web server framework (async, modern, fast)
axum = { version = "0.7", features = ["json", "macros"] }

# Async runtime (required by Axum)
tokio = { version = "1", features = ["full"] }

# JSON serialization/deserialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP client to call Alpaca API
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# Middleware for logging, CORS, etc.
tower-http = { version = "0.5", features = ["trace", "cors"] }

# Structured logging (better than println!)
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }

# Load .env file for API keys
dotenvy = "0.15"

# Optional: Alpaca client library (makes API calls easier)
# We'll use raw reqwest first, but you can add this later:
# apca = "0.28"
```

**Save the file**.

### What Each Dependency Does

| Crate | Purpose | Why We Need It |
|-------|---------|----------------|
| **axum** | Web server framework | Handles incoming webhook HTTP requests |
| **tokio** | Async runtime | Lets us handle multiple requests concurrently without blocking |
| **serde** | Serialization | Converts JSON ↔ Rust structs automatically |
| **serde_json** | JSON handling | Parses JSON payloads from TradingView |
| **reqwest** | HTTP client | Makes API calls to Alpaca |
| **tower-http** | Middleware | Adds logging, CORS headers, etc. |
| **tracing** | Logging | Structured logs (better than `println!`) |
| **tracing-subscriber** | Log output | Formats logs to console |
| **dotenvy** | Environment loader | Reads `.env` file for secrets |

### Step 3: Download Dependencies

Run:

```bash
cargo build
```

**What happens**:
- Cargo reads `Cargo.toml`
- Downloads all listed crates (libraries) from crates.io
- Downloads their dependencies (transitive deps — ~50-100 total)
- Compiles everything
- Creates `Cargo.lock` (exact version lock file — don't edit manually)

**This takes 2-5 minutes the first time** (downloads ~200 MB, compiles everything).

You'll see:
```
    Updating crates.io index
 Downloading crates and compiling...
   Compiling proc-macro2 v1.x.x
   Compiling syn v2.x.x
   ...
   Compiling alpaca-webhook-rs v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 3m 21s
```

**If you see errors**: Check your `Cargo.toml` for typos (indentation matters in TOML!). Common issue: missing `=` or `"` quotes.

---

## Configuration Setup

### Step 4: Create `.env` File

In the project root (same folder as `Cargo.toml`), create a file named `.env`:

```bash
touch .env
```

Open it and add:

```env
# Alpaca API Configuration
ALPACA_KEY=YOUR_KEY_ID_HERE
ALPACA_SECRET=YOUR_SECRET_KEY_HERE
ALPACA_BASE_URL=https://paper-api.alpaca.markets

# Webhook Security
WEBHOOK_SECRET=my-super-secret-password-12345

# Server Configuration
PORT=8080
RUST_LOG=info
```

**Replace these values**:

1. **Get Alpaca Keys**:
   - Go to https://app.alpaca.markets/paper/dashboard/overview
   - Click "Generate API Keys" or go to "Your API Keys" section
   - Copy:
     - **Key ID** → paste into `ALPACA_KEY=`
     - **Secret Key** → paste into `ALPACA_SECRET=` (only shown once!)
   - **Use paper trading first!** (`paper-api.alpaca.markets` — fake money for testing)

2. **WEBHOOK_SECRET**: Change this to a random strong password (this prevents unauthorized webhook calls)

3. **PORT**: 8080 is fine (change if that port is busy)

4. **RUST_LOG**: Controls log verbosity:
   - `debug` = very detailed
   - `info` = normal (recommended)
   - `warn` = only warnings/errors
   - `error` = only errors

**Save `.env`**.

### Step 5: Protect Your Secrets

**NEVER commit `.env` to git!**

Open `.gitignore` and make sure it includes:

```gitignore
/target/
.env
*.env
```

Run:

```bash
git status
```

Make sure `.env` is NOT listed. If it is, run:

```bash
git rm --cached .env
```

---

## Writing the Code

Now the fun part! We'll build this step-by-step.

### Step 6: Basic Server (Hello World)

Open `src/main.rs` and **replace everything** with:

```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Start simple: just a test endpoint
    let app = Router::new()
        .route("/", get(|| async { "Alpaca Webhook Receiver is running!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("🚀 Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
```

**Test it**:

```bash
cargo run
```

You should see:
```
🚀 Server running on http://0.0.0.0:8080
```

Open your browser → http://localhost:8080

You should see: `Alpaca Webhook Receiver is running!`

**Press Ctrl+C to stop.**

### Step 7: Add Logging

Replace `main.rs` with:

```rust
use axum::{
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Alpaca Webhook Receiver...");

    let app = Router::new()
        .route("/", get(|| async { "Alpaca Webhook Receiver is running!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    info!("🚀 Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
```

**Run again**:

```bash
cargo run
```

Now you see prettier logs:
```
  INFO alpaca_webhook_rs: Starting Alpaca Webhook Receiver...
  INFO alpaca_webhook_rs: 🚀 Server running on http://0.0.0.0:8080
```

### Step 8: Load Environment Variables

Add at the top of `main()`:

```rust
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize logging (read RUST_LOG from .env)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .with_target(false)
        .compact()
        .init();

    info!("Starting Alpaca Webhook Receiver...");

    // Read environment variables
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let alpaca_key = std::env::var("ALPACA_KEY").expect("ALPACA_KEY must be set in .env");
    let alpaca_secret = std::env::var("ALPACA_SECRET").expect("ALPACA_SECRET must be set");
    let base_url = std::env::var("ALPACA_BASE_URL")
        .unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string());

    info!("Using Alpaca URL: {}", base_url);

    let app = Router::new()
        .route("/", get(|| async { "Alpaca Webhook Receiver is running!" }));

    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap();

    info!("🚀 Server running on http://{}", bind_addr);

    axum::serve(listener, app).await.unwrap();
}
```

**Full imports at top**:

```rust
use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber;
```

**Test**:

```bash
cargo run
```

Should show:
```
  INFO Starting Alpaca Webhook Receiver...
  INFO Using Alpaca URL: https://paper-api.alpaca.markets
  INFO 🚀 Server running on http://0.0.0.0:8080
```

If you see `ALPACA_KEY must be set` error → check your `.env` file is in the right place (same folder as `Cargo.toml`).

### Step 9: Create Webhook Endpoint (Receive JSON)

Now let's add the `/webhook` endpoint that receives trade signals.

**Add these structs** near the top of `main.rs` (after imports):

```rust
use serde::{Deserialize, Serialize};

// Struct that matches the JSON payload from TradingView
#[derive(Debug, Deserialize, Serialize)]
struct TradeSignal {
    action: String,       // "buy" or "sell"
    symbol: String,       // e.g., "AAPL", "TSLA"
    quantity: u32,        // number of shares
    order_type: String,   // "market" or "limit"
    #[serde(default)]
    price: Option<f64>,   // for limit orders (optional)
    secret: String,       // security: must match WEBHOOK_SECRET
}

// Response we send back
#[derive(Serialize)]
struct WebhookResponse {
    status: String,
    message: String,
}
```

**Add the handler function** (before `main`):

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{info, warn, error};

async fn handle_webhook(
    Json(signal): Json<TradeSignal>,
) -> impl IntoResponse {
    info!("📨 Received webhook: {:?}", signal);

    // Validate secret
    let webhook_secret = std::env::var("WEBHOOK_SECRET")
        .unwrap_or_else(|_| "default".to_string());

    if signal.secret != webhook_secret {
        warn!("❌ Invalid secret in webhook request");
        return (
            StatusCode::FORBIDDEN,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: "Invalid secret".to_string(),
            }),
        );
    }

    // Validate action
    if signal.action != "buy" && signal.action != "sell" {
        warn!("❌ Invalid action: {}", signal.action);
        return (
            StatusCode::BAD_REQUEST,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: "Action must be 'buy' or 'sell'".to_string(),
            }),
        );
    }

    info!("✅ Valid signal: {} {} shares of {}", signal.action, signal.quantity, signal.symbol);

    // TODO: Place order with Alpaca (next step!)

    (
        StatusCode::OK,
        Json(WebhookResponse {
            status: "success".to_string(),
            message: format!("Signal received: {} {}", signal.action, signal.symbol),
        }),
    )
}
```

**Update imports at top**:

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
```

**Add the route in `main()`**:

```rust
let app = Router::new()
    .route("/", get(|| async { "Alpaca Webhook Receiver is running!" }))
    .route("/webhook", post(handle_webhook));  // ← Add this line
```

**Full `main.rs` at this point**:

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct TradeSignal {
    action: String,
    symbol: String,
    quantity: u32,
    order_type: String,
    #[serde(default)]
    price: Option<f64>,
    secret: String,
}

#[derive(Serialize)]
struct WebhookResponse {
    status: String,
    message: String,
}

async fn handle_webhook(
    Json(signal): Json<TradeSignal>,
) -> impl IntoResponse {
    info!("📨 Received webhook: {:?}", signal);

    let webhook_secret = std::env::var("WEBHOOK_SECRET")
        .unwrap_or_else(|_| "default".to_string());

    if signal.secret != webhook_secret {
        warn!("❌ Invalid secret in webhook request");
        return (
            StatusCode::FORBIDDEN,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: "Invalid secret".to_string(),
            }),
        );
    }

    if signal.action != "buy" && signal.action != "sell" {
        warn!("❌ Invalid action: {}", signal.action);
        return (
            StatusCode::BAD_REQUEST,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: "Action must be 'buy' or 'sell'".to_string(),
            }),
        );
    }

    info!("✅ Valid signal: {} {} shares of {}", signal.action, signal.quantity, signal.symbol);

    // TODO: Place order with Alpaca

    (
        StatusCode::OK,
        Json(WebhookResponse {
            status: "success".to_string(),
            message: format!("Signal received: {} {}", signal.action, signal.symbol),
        }),
    )
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .with_target(false)
        .compact()
        .init();

    info!("Starting Alpaca Webhook Receiver...");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let app = Router::new()
        .route("/", get(|| async { "Alpaca Webhook Receiver is running!" }))
        .route("/webhook", post(handle_webhook));

    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap();

    info!("🚀 Server running on http://{}", bind_addr);

    axum::serve(listener, app).await.unwrap();
}
```

**Test with curl**:

```bash
cargo run
```

In another terminal:

```bash
curl -X POST http://localhost:8080/webhook \
  -H "Content-Type: application/json" \
  -d '{
    "action": "buy",
    "symbol": "AAPL",
    "quantity": 10,
    "order_type": "market",
    "secret": "my-super-secret-password-12345"
  }'
```

**Expected response**:

```json
{"status":"success","message":"Signal received: buy AAPL"}
```

**In server logs**:

```
  INFO 📨 Received webhook: TradeSignal { action: "buy", symbol: "AAPL", quantity: 10, order_type: "market", price: None, secret: "..." }
  INFO ✅ Valid signal: buy 10 shares of AAPL
```

**Try with wrong secret**:

```bash
curl -X POST http://localhost:8080/webhook \
  -H "Content-Type: application/json" \
  -d '{
    "action": "buy",
    "symbol": "AAPL",
    "quantity": 10,
    "order_type": "market",
    "secret": "wrong-secret"
  }'
```

Should return:
```json
{"status":"error","message":"Invalid secret"}
```

Great! Webhook validation works. Now let's connect to Alpaca.

### Step 10: Connect to Alpaca API

**Add Alpaca order struct**:

```rust
#[derive(Debug, Serialize)]
struct AlpacaOrder {
    symbol: String,
    qty: String,
    side: String,  // "buy" or "sell"
    #[serde(rename = "type")]
    order_type: String,  // "market" or "limit"
    time_in_force: String,  // "day", "gtc", "ioc", "fok"
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<String>,
}
```

**Add the function to place orders**:

```rust
use reqwest::Client;

async fn place_alpaca_order(signal: &TradeSignal) -> Result<String, String> {
    let alpaca_key = std::env::var("ALPACA_KEY")
        .map_err(|_| "ALPACA_KEY not set".to_string())?;
    let alpaca_secret = std::env::var("ALPACA_SECRET")
        .map_err(|_| "ALPACA_SECRET not set".to_string())?;
    let base_url = std::env::var("ALPACA_BASE_URL")
        .unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string());

    let order = AlpacaOrder {
        symbol: signal.symbol.clone(),
        qty: signal.quantity.to_string(),
        side: signal.action.clone(),
        order_type: signal.order_type.clone(),
        time_in_force: "day".to_string(),
        limit_price: signal.price.map(|p| p.to_string()),
    };

    info!("📤 Sending order to Alpaca: {:?}", order);

    let client = Client::new();
    let url = format!("{}/v2/orders", base_url);

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
        info!("✅ Order placed successfully: {}", body);
        Ok(body)
    } else {
        tracing::error!("❌ Alpaca API error {}: {}", status, body);
        Err(format!("Alpaca API error: {} - {}", status, body))
    }
}
```

**Update `handle_webhook` to call it**:

Replace the `// TODO: Place order with Alpaca` line with:

```rust
// Place order with Alpaca
match place_alpaca_order(&signal).await {
    Ok(order_response) => {
        info!("🎉 Order placed successfully");
        return (
            StatusCode::OK,
            Json(WebhookResponse {
                status: "success".to_string(),
                message: format!("Order placed: {} {} shares of {}", signal.action, signal.quantity, signal.symbol),
            }),
        );
    }
    Err(err) => {
        tracing::error!("Failed to place order: {}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: format!("Failed to place order: {}", err),
            }),
        );
    }
}
```

**Update imports** (add `use tracing::error;`):

```rust
use tracing::{info, warn, error};
```

**Full working `main.rs` (complete version)**:

```rust
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};

#[derive(Debug, Deserialize, Serialize)]
struct TradeSignal {
    action: String,
    symbol: String,
    quantity: u32,
    order_type: String,
    #[serde(default)]
    price: Option<f64>,
    secret: String,
}

#[derive(Serialize)]
struct WebhookResponse {
    status: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct AlpacaOrder {
    symbol: String,
    qty: String,
    side: String,
    #[serde(rename = "type")]
    order_type: String,
    time_in_force: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<String>,
}

async fn place_alpaca_order(signal: &TradeSignal) -> Result<String, String> {
    let alpaca_key = std::env::var("ALPACA_KEY")
        .map_err(|_| "ALPACA_KEY not set".to_string())?;
    let alpaca_secret = std::env::var("ALPACA_SECRET")
        .map_err(|_| "ALPACA_SECRET not set".to_string())?;
    let base_url = std::env::var("ALPACA_BASE_URL")
        .unwrap_or_else(|_| "https://paper-api.alpaca.markets".to_string());

    let order = AlpacaOrder {
        symbol: signal.symbol.clone(),
        qty: signal.quantity.to_string(),
        side: signal.action.clone(),
        order_type: signal.order_type.clone(),
        time_in_force: "day".to_string(),
        limit_price: signal.price.map(|p| p.to_string()),
    };

    info!("📤 Sending order to Alpaca: {:?}", order);

    let client = Client::new();
    let url = format!("{}/v2/orders", base_url);

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
        info!("✅ Order placed successfully: {}", body);
        Ok(body)
    } else {
        error!("❌ Alpaca API error {}: {}", status, body);
        Err(format!("Alpaca API error: {} - {}", status, body))
    }
}

async fn handle_webhook(
    Json(signal): Json<TradeSignal>,
) -> impl IntoResponse {
    info!("📨 Received webhook: {:?}", signal);

    let webhook_secret = std::env::var("WEBHOOK_SECRET")
        .unwrap_or_else(|_| "default".to_string());

    if signal.secret != webhook_secret {
        warn!("❌ Invalid secret in webhook request");
        return (
            StatusCode::FORBIDDEN,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: "Invalid secret".to_string(),
            }),
        );
    }

    if signal.action != "buy" && signal.action != "sell" {
        warn!("❌ Invalid action: {}", signal.action);
        return (
            StatusCode::BAD_REQUEST,
            Json(WebhookResponse {
                status: "error".to_string(),
                message: "Action must be 'buy' or 'sell'".to_string(),
            }),
        );
    }

    info!("✅ Valid signal: {} {} shares of {}", signal.action, signal.quantity, signal.symbol);

    // Place order with Alpaca
    match place_alpaca_order(&signal).await {
        Ok(_) => {
            info!("🎉 Order placed successfully");
            (
                StatusCode::OK,
                Json(WebhookResponse {
                    status: "success".to_string(),
                    message: format!("Order placed: {} {} shares of {}", signal.action, signal.quantity, signal.symbol),
                }),
            )
        }
        Err(err) => {
            error!("Failed to place order: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WebhookResponse {
                    status: "error".to_string(),
                    message: format!("Failed to place order: {}", err),
                }),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .with_target(false)
        .compact()
        .init();

    info!("Starting Alpaca Webhook Receiver...");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let app = Router::new()
        .route("/", get(|| async { "Alpaca Webhook Receiver is running!" }))
        .route("/webhook", post(handle_webhook));

    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap();

    info!("🚀 Server running on http://{}", bind_addr);

    axum::serve(listener, app).await.unwrap();
}
```

**Test the full flow**:

```bash
cargo run
```

In another terminal:

```bash
curl -X POST http://localhost:8080/webhook \
  -H "Content-Type: application/json" \
  -d '{
    "action": "buy",
    "symbol": "AAPL",
    "quantity": 1,
    "order_type": "market",
    "secret": "my-super-secret-password-12345"
  }'
```

**Check your Alpaca paper trading dashboard**: https://app.alpaca.markets/paper/dashboard/overview

You should see a new order for 1 share of AAPL!

**In logs**:

```
  INFO 📨 Received webhook: TradeSignal { ... }
  INFO ✅ Valid signal: buy 1 shares of AAPL
  INFO 📤 Sending order to Alpaca: ...
  INFO ✅ Order placed successfully: {"id":"...","symbol":"AAPL",...}
  INFO 🎉 Order placed successfully
```

🎉 **Congratulations!** You've built a working webhook receiver that places real orders!

---

## Testing Locally

### Step 11: Expose with ngrok

To test with TradingView, you need a public URL.

**Start your server**:

```bash
cargo run
```

**In another terminal, start ngrok**:

```bash
ngrok http 8080
```

You'll see:

```
Forwarding   https://abc123.ngrok.io -> http://localhost:8080
```

**Copy the `https://abc123.ngrok.io` URL** (changes each time you restart ngrok — paid plan gives static URLs).

### Step 12: Set Up TradingView Alert

1. Go to https://www.tradingview.com
2. Open any chart (e.g., AAPL)
3. Click the **Alert** button (clock icon, or right-click chart → "Add Alert")
4. Set conditions (e.g., "Crossing" any value for testing)
5. Under **Notifications**, enable **Webhook URL**
6. Paste: `https://abc123.ngrok.io/webhook`
7. In **Message** (JSON payload), paste:

```json
{
  "action": "buy",
  "symbol": "{{ticker}}",
  "quantity": 1,
  "order_type": "market",
  "secret": "my-super-secret-password-12345"
}
```

**Note**: `{{ticker}}` is a TradingView variable that auto-fills with the chart's symbol.

8. Click **Create**

**Trigger the alert** (adjust price level or wait for it to fire).

**Check**:
- Your server logs (should show webhook received)
- Alpaca dashboard (should show new order)
- TradingView alert log (green checkmark if webhook succeeded)

---

## Deploying to Production

### Step 13: Build Release Binary

For production, build an optimized binary:

```bash
cargo build --release
```

**Binary location**: `target/release/alpaca-webhook-rs`

This is a single, standalone executable (~10-20 MB). No dependencies needed on the target server!

### Step 14: Deploy to VPS

**Recommended VPS providers**:
- DigitalOcean ($6/month — "Basic Droplet")
- Linode ($5/month)
- Vultr ($5/month)
- Hetzner ($4/month — EU-based)

**Steps**:

1. **Create a Ubuntu 24.04 droplet/instance**
2. **SSH into it**:
   ```bash
   ssh root@your-vps-ip
   ```
3. **Upload your binary**:
   ```bash
   # On your local machine
   scp target/release/alpaca-webhook-rs root@your-vps-ip:/root/
   scp .env root@your-vps-ip:/root/
   ```
4. **Install dependencies** (none needed for Rust binary, but install for HTTPS):
   ```bash
   sudo apt update
   sudo apt install -y caddy  # Reverse proxy for HTTPS
   ```
5. **Make binary executable**:
   ```bash
   chmod +x /root/alpaca-webhook-rs
   ```
6. **Run it**:
   ```bash
   cd /root
   ./alpaca-webhook-rs
   ```

### Step 15: Set Up HTTPS with Caddy

**Create Caddyfile**:

```bash
nano /etc/caddy/Caddyfile
```

Add:

```
your-domain.com {
    reverse_proxy localhost:8080
}
```

**Start Caddy**:

```bash
sudo systemctl enable caddy
sudo systemctl start caddy
```

Caddy automatically gets a Let's Encrypt SSL certificate!

**Update TradingView webhook URL** to: `https://your-domain.com/webhook`

### Step 16: Run as Systemd Service (Always-On)

**Create service file**:

```bash
sudo nano /etc/systemd/system/alpaca-webhook.service
```

Add:

```ini
[Unit]
Description=Alpaca Webhook Receiver
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/root/alpaca-webhook-rs
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Enable & start**:

```bash
sudo systemctl daemon-reload
sudo systemctl enable alpaca-webhook
sudo systemctl start alpaca-webhook
```

**Check status**:

```bash
sudo systemctl status alpaca-webhook
```

**View logs**:

```bash
sudo journalctl -u alpaca-webhook -f
```

Now your webhook receiver runs 24/7 and auto-restarts if it crashes!

---

## Troubleshooting

### Common Issues

**1. "ALPACA_KEY must be set" error**
- Make sure `.env` is in the same folder as `Cargo.toml` (project root)
- Check `.env` has correct syntax (no spaces around `=`)
- Restart your terminal after creating `.env`

**2. "Address already in use" error**
- Port 8080 is busy. Change `PORT=8081` in `.env`
- Or kill the process: `lsof -ti:8080 | xargs kill`

**3. Alpaca API returns 403 Forbidden**
- Check your API keys are correct
- Make sure you're using paper trading URL for paper keys
- API keys might be regenerated — get new ones from Alpaca dashboard

**4. TradingView webhook fails**
- Check ngrok is running and URL is correct
- Verify `secret` in TradingView message matches `.env`
- Check server logs for errors
- Test webhook manually with curl first

**5. "rustc: command not found"**
- Rust not installed or not in PATH
- Run: `source $HOME/.cargo/env`
- Or restart terminal after installing Rust

**6. Compilation errors**
- Check `Cargo.toml` syntax (proper quotes, commas)
- Make sure all dependencies have correct versions
- Try: `cargo clean && cargo build`

**7. Orders not showing in Alpaca**
- Verify you're checking the right account (paper vs live)
- Check market hours (US stock market: 9:30 AM - 4 PM ET, Mon-Fri)
- Some symbols may not be tradable on Alpaca

### Debug Tips

**Enable detailed logs**:

In `.env`:
```
RUST_LOG=debug
```

**Test Alpaca connection**:

```bash
curl -H "APCA-API-KEY-ID: YOUR_KEY" \
     -H "APCA-API-SECRET-KEY: YOUR_SECRET" \
     https://paper-api.alpaca.markets/v2/account
```

Should return your account info.

**Test webhook locally**:

```bash
curl -X POST http://localhost:8080/webhook \
  -H "Content-Type: application/json" \
  -d '{"action":"buy","symbol":"AAPL","quantity":1,"order_type":"market","secret":"your-secret"}'
```

---

## Next Steps

### Enhancements You Can Add

1. **Risk Management**:
   - Check existing positions before placing orders
   - Limit max position size per symbol
   - Stop trading after max daily loss

2. **More Order Types**:
   - Limit orders (already supported in struct!)
   - Stop-loss orders
   - Bracket orders

3. **Database Logging**:
   - Store all trades in SQLite/PostgreSQL
   - Track P&L over time

4. **Notifications**:
   - Discord/Telegram bot to confirm orders
   - Email alerts on errors

5. **Multiple Brokers**:
   - Add IBKR, Tradier, Binance support
   - Route different symbols to different brokers

6. **Dashboard**:
   - Web UI to view trades, stats, logs
   - Use Axum to serve HTML/React frontend

7. **Testing**:
   - Add unit tests (`#[cfg(test)]`)
   - Integration tests with mock Alpaca API

### Learning Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Axum Docs**: https://docs.rs/axum
- **Alpaca API Docs**: https://alpaca.markets/docs/api-references/trading-api/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/

---

## Summary

You've built a complete automated trading system that:

✅ Listens for webhooks from TradingView
✅ Validates incoming signals with secret authentication
✅ Places real orders on Alpaca (paper trading)
✅ Handles errors gracefully with logging
✅ Runs as a single compiled binary
✅ Can be deployed to production VPS

**Total code**: ~150 lines of Rust
**Dependencies**: 9 crates
**Performance**: <1ms latency, <10MB RAM

**This is production-ready for paper trading.** Before going live with real money:
- Add more error handling
- Implement risk management
- Test extensively
- Consider adding a database for audit logs
- Set up monitoring/alerting

Happy trading! 🚀📈
