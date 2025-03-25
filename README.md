# rtpx (RTP)
Safe Rust bindings for CTP and its variations

# 🦀 rtpx: Safe Rust Bindings for CTP Trading System and its variations

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`rtpx` provides safe and ergonomic Rust bindings for the CTP (Comprehensive Transaction Platform) trading system and its variants (ATP, XTP), widely used in Chinese financial markets. This project aims to bring Rust's safety and performance guarantees to the CTP ecosystem while maintaining compatibility with various CTP-compatible implementations.

## Supported SDK versions

|sdk|version|original URL|
|--|--|--|
|CTP|6.7.7_210240607|http://www.sfit.com.cn/DocumentDown/api_3/5_2_2/v6.7.7_traderapi_20240607.zip|
|ATP|6.3.15|N/A|

## 🌟 Features

- **Safe Abstractions**: Zero-cost abstractions over CTP's C++ interfaces with Rust's safety guarantees
- **Multiple CTP Variants**: Support for CTP, ATP and XTP through feature flags
- **Error Handling**: Rust-native error handling replacing C++ exceptions
- **Cross-Platform**: Windows and Linux support
- **Zero Overhead**: Direct FFI bindings with no runtime cost
- **Type Safety**: Strong typing for market data and trading interfaces

## 🚀 Basic Usage

```rust
use std::ffi::CString;
use std::thread;
use std::time::Duration;

// Import the RTP trader API with required features
use rtp::trader::{GenericTraderApi, TraderApi, TraderSpi, ResumeType};
use rtp::common::{DisconnectionReason, RspResult};
use rtp::binding::*;

// Create a simple TraderSpi implementation
struct MyTraderSpi {
    connected: bool,
}

impl MyTraderSpi {
    fn new() -> Self {
        MyTraderSpi { connected: false }
    }
}

impl TraderSpi for MyTraderSpi {
    fn on_front_connected(&mut self) {
        println!("Connected to trading server");
        self.connected = true;
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("Disconnected from trading server: {:?}", reason);
        self.connected = false;
    }
}

fn main() {
    // Create a new instance of TraderApi
    let flow_path = CString::new("./flow_path").unwrap();
    let mut trader = TraderApi::new(flow_path);
    
    // Register SPI for callbacks
    let trader_spi = Box::new(MyTraderSpi::new());
    trader.register_spi(trader_spi);
    
    // Configure the trader API
    trader.subscribe_public_topic(ResumeType::Quick);
    trader.subscribe_private_topic(ResumeType::Quick);
    
    // Register a trading server front
    trader.register_front(CString::new("tcp://trading.server.address:port").unwrap());
    
    // Initialize API (non-blocking call)
    trader.init();
    
    // Allow some time for connection and processing
    thread::sleep(Duration::from_secs(5));
}
```

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rtpx = "1.0.3"
```

By default, the CTP variant is enabled. To use ATP or XTP instead:

```toml
[dependencies]
rtpx = { version = "1.0.3", default-features = false, features = ["atp"] }
```

## 🔧 Supported CTP Variants

Feature flags allow you to choose which implementation to use:

- `ctp` - Standard CTP implementation (default)
- `atp` - ATP implementation
- `xtp` - XTP implementation

Only one variant can be enabled at a time.

## 🛠️ Building from Source

```bash
# Clone the repository
git clone https://github.com/glacierx/RTP
cd RTP

# Build with CTP (default)
cargo build --release

# Build with ATP
cargo build --release --no-default-features --features=atp

# Run tests
cargo test
```

## 🧪 Testing

The library includes tests showing basic usage patterns for CTP and ATP. These are non-connecting tests that demonstrate API initialization and proper structure but don't connect to real servers.

To run the tests:

```bash
# Run CTP tests (default)
cargo test

# Run ATP tests
cargo test --no-default-features --features=atp
```

## ⚠️ Note on Writing Tests

When writing your own tests, be aware that the import structure uses:

```rust
use rtp::trader::{GenericTraderApi, TraderApi, TraderSpi, ResumeType};
use rtp::common::{DisconnectionReason, RspResult};
use rtp::binding::*;
```

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
