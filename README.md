# Alpaca Webhook Receiver (Rust)

A lightweight, high-performance webhook receiver built in Rust that bridges TradingView strategy alerts (or similar signal providers) to the Alpaca Markets trading API. This project was created to learn Rust while building a reliable, production-ready automated trading solution.

## What is This and Why Do I Need It?

### The Problem

Most trading platforms like TradingView, TrendSpider, or custom indicators can send instant notifications when your trading conditions are met (e.g., "buy when price crosses above moving average"). However, **broker APIs like Alpaca don't accept incoming webhooks directly** — they only let you call their API to place orders.

### The Solution: A Webhook Receiver

A webhook receiver is a small web server that acts as the "glue" between your trading signals and your broker:

1. **TradingView alert fires** → sends webhook to your server (e.g., `https://your-server.com/webhook`)
2. **Your receiver gets the POST request** with trade signal (symbol, action, quantity)
3. **Your code validates and processes** the signal
4. **Authenticates to Alpaca** using your API keys
5. **Places the actual trade** via Alpaca's REST API
6. **(Optional)** Sends confirmation to Discord/Telegram/email

### Why Rust?

- **Reliable**: Memory safety guarantees = fewer crashes in production
- **Fast**: Compiled binary with minimal latency
- **Efficient**: Tiny resource footprint (perfect for VPS or Raspberry Pi)
- **Single Binary**: Easy deployment, no dependency hell
- **"Set it and forget it"**: Built for 24/7 automated trading

### Common Full Automation Flow

```
[TradingView Strategy]
        ↓ (alert triggers)
[Webhook POST to your URL]
        ↓ (JSON payload: symbol, action, quantity)
[This Rust Server - Webhook Receiver]
        ↓ (validates, authenticates)
[Alpaca API Call]
        ↓
[Order Executed on Market]
```

## Features

- ✅ **Async HTTP server** using Axum (modern, fast)
- ✅ **JSON payload validation** using Serde
- ✅ **Secure webhook authentication** with secret key checks
- ✅ **Alpaca integration** via REST API v2
- ✅ **Paper trading support** for safe testing
- ✅ **Structured logging** with tracing
- ✅ **Environment-based config** (`.env` file for API keys)
- ✅ **Low resource usage** (~5-10 MB memory)
- ✅ **Single compiled binary** for easy deployment
- ✅ **Market & limit order support**

## Why "Locally Hosted"?

Running this on your own machine/VPS gives you:
- **Full control**: Custom risk checks (e.g., "don't trade if position > 5%")
- **Security**: Your API keys never leave your control
- **Flexibility**: Connect to multiple brokers, add logging, notifications, etc.
- **Cost**: No monthly SaaS fees (unlike TradersPost, Alertatron, etc.)

**Note**: For real money trading, run on a VPS ($5-15/month on DigitalOcean/Linode/Vultr) instead of your laptop to avoid missed trades from sleep mode, internet drops, or power outages.

## Alternatives to Self-Hosting

If you don't want to code/host yourself:
- **Third-party bridges**: TradersPost, Alertatron, Option Alpha, Cornix (crypto), PineConnector (MT4/5) — they host for you (usually monthly fee)
- **Serverless**: AWS Lambda + API Gateway (still requires setup)

But you're here to learn Rust and have full control — so let's build it!

## Quick Start

See [SETUP_GUIDE.md](SETUP_GUIDE.md) for detailed step-by-step instructions on:
- Installing Rust and dependencies
- Writing the webhook receiver code
- Configuring Alpaca API keys
- Testing with ngrok and TradingView
- Deploying to production VPS

## Project Structure

```
alpaca-webhook-rs/
├── src/
│   └── main.rs           # Main webhook receiver code
├── .env                  # API keys and config (never commit!)
├── .gitignore
├── Cargo.toml            # Rust dependencies
├── Cargo.lock            # Exact versions (auto-generated)
├── README.md             # This file
└── SETUP_GUIDE.md        # Step-by-step coding guide
```

## Security Warning

⚠️ **Never commit your `.env` file or API keys to GitHub!** This codebase is for educational purposes and personal use. Always test with paper trading first before going live with real money.

## License

MIT (or choose your preferred license)

## Acknowledgments

Built to learn Rust and automated trading. Inspired by the DIY trading community's need for reliable, low-cost webhook bridges.

---
**Ready to code?** → Open [SETUP_GUIDE.md](SETUP_GUIDE.md) and let's build this step by step!
