# rtp
Rust binding for CTP and it's variations

# 🦀 rtp: Safe Rust Bindings for CTP Trading System(and it's variations)

[![Crates.io](https://img.shields.io/crates/v/rtp)](https://crates.io/crates/rtp)
[![Documentation](https://docs.rs/rtp/badge.svg)](https://docs.rs/rtp)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`rtp` provides safe and ergonomic Rust bindings for the CTP (Comprehensive Transaction Platform) trading system, widely used in Chinese financial markets. This project aims to bring Rust's safety and performance guarantees to the CTP ecosystem while maintaining compatibility with various CTP-compatible implementations.

## CTP versions

|sdk|version|original URL|
|--|--|--|
|CTP|6.7.7_210240607|http://www.sfit.com.cn/DocumentDown/api_3/5_2_2/v6.7.7_traderapi_20240607.zip|
|ATP|6.3.15|N/A|

## 🌟 Features

- **Safe Abstractions**: Zero-cost abstractions over CTP's C++ interfaces with Rust's safety guarantees
- **Multiple CTP Variants**: Support for different CTP implementations across brokers and markets
- **Error Handling**: Rust-native error handling replacing C++ exceptions
- **Async Support**: Modern async/await API for event handling
- **Cross-Platform**: Windows and Linux support
- **Zero Overhead**: Direct FFI bindings with no runtime cost
- **Type Safety**: Strong typing for market data and trading interfaces

## 🚀 Quick Start

```rust
use rust_ctp::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new CTP client
    let mut client = CtpTradingClient::new()
        .front_addr("tcp://180.168.146.187:10130")
        .broker_id("9999")
        .build()?;

    // Connect and login
    client.connect().await?;
    client.login("YOUR_USERNAME", "YOUR_PASSWORD").await?;

    // Subscribe to market data
    client.subscribe(&["rb2410"]).await?;

    // Handle market data
    while let Some(data) = client.next_tick().await {
        println!("Received tick: {:?}", data);
    }

    Ok(())
}
```

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rtp = "0.1.0"
```

## 🔧 Supported CTP Variants

- CTP Standard Version (CTPSE)
- CTPMIN (Simulated Trading)
- Simnow Trading Platform
- Custom broker implementations (extensible)

## 🛠️ Building from Source

```bash
# Clone the repository
git clone https://github.com/glacierx/rtp
cd rtp

# Build the project
cargo build --release

# Run tests
cargo test
```

## 📚 Documentation

For detailed documentation and examples, visit [docs.rs/rtp](https://docs.rs/rtp).

The documentation includes:
- Complete API reference
- Trading examples
- Market data handling
- Error handling patterns
- Best practices

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## ⚠️ Disclaimer

This software is provided "as is", without warranty of any kind. Trading in financial markets carries significant risk. Make sure to test thoroughly in a simulated environment before using in production.

## 📄 License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- CTP development team for the original C++ SDK
- Rust FFI community for guidance and tools
- Contributors who helped make this project possible

---

*Note: This is an unofficial Rust binding. CTP is a trademark of Shanghai Futures Exchange.*
