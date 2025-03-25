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
use rtp::trader::{DisconnectionReason, RspResult};

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

## 🔄 Using Different SDK Versions

This package supports multiple versions of CTP and ATP SDKs. Here's how to use different versions:

### Using a Different CTP Version

1. Download your desired CTP version from the official website (http://www.sfit.com.cn/)
2. Place the SDK files in your project:
   ```bash
   your_project/
   ├── sdk/
   │   └── ctp/
   │       ├── include/
   │       │   ├── ThostFtdcMdApi.h
   │       │   ├── ThostFtdcTraderApi.h
   │       │   └── ThostFtdcUserApiStruct.h
   │       └── lib/
   │           ├── libthostmduserapi_se.so      # Linux
   │           └── libthosttraderapi_se.so      # Linux
   ```

3. Update your project's build configuration:
   ```toml
   [dependencies]
   rtpx = { version = "1.0.4", default-features = false, features = ["ctp"] }

   [package.metadata.rtpx]
   ctp_sdk_path = "sdk/ctp"  # Path to your CTP SDK
   ```

### Using a Different ATP Version

1. Obtain the ATP SDK files from your broker
2. Place the SDK files similarly:
   ```bash
   your_project/
   ├── sdk/
   │   └── atp/
   │       ├── include/
   │       │   ├── AtpTraderApi.h
   │       │   └── AtpUserApiStruct.h
   │       └── lib/
   │           ├── libatptraderapi.so           # Linux
   │           └── libatpmduserapi.so           # Linux
   ```

3. Update your project's build configuration:
   ```toml
   [dependencies]
   rtpx = { version = "1.0.4", default-features = false, features = ["atp"] }

   [package.metadata.rtpx]
   atp_sdk_path = "sdk/atp"  # Path to your ATP SDK
   ```

### Important Notes

- Always ensure binary compatibility between your chosen SDK version and your broker's trading system
- Test thoroughly with your specific SDK version in a simulation environment before using in production
- Some features might vary between different SDK versions
- When upgrading SDK versions, review the changelog for breaking changes
- The package's default SDK versions are tested and known to work, but you can override them as shown above

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

## 📚 Running Examples

The repository includes examples demonstrating basic usage of the trading API. The most comprehensive example is `basic_usage.rs`, which demonstrates connecting to a trading server, logging in, and querying account information and positions.

To run the examples:

```bash
# Run the basic usage example with CTP (default)
cargo run --example basic_usage

# Run the basic usage example with ATP
cargo run --example basic_usage --no-default-features --features=atp

# Run the basic usage example with XTP
cargo run --example basic_usage --no-default-features --features=xtp
```

### Configuring the Examples

Before running the examples, you'll need to modify the connection parameters in the source code:

1. Open the example file (e.g., `examples/basic_usage.rs`)
2. Update the following values with your actual credentials:
   ```rust
   let front_address = "tcp://180.168.146.187:10130"; // Change to your server
   let broker_id = "9999";                            // Change to your broker ID
   let investor_id = "your_investor_id";              // Change to your actual ID
   let password = "your_password";                    // Change to your actual password
   ```

### Note on Flow Path Files

The examples create flow files in the current directory. These files are used by the CTP/ATP/XTP APIs to store session information. The default flow path is `./flow_path`, but you can change it if needed.

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
use rtp::trader::{DisconnectionReason, RspResult};
```

## 🔍 Troubleshooting

### Working with CTP Enum Values

CTP/ATP/XTP have many enum values that are defined in their C++ headers. In the Rust binding, these are often represented as primitive types (like `u8`) with specific values. When working with these values, you may need to use the raw numeric values rather than trying to access enum variants.

For example, when dealing with position direction (TThostFtdcPosiDirectionType):

```rust
// Correct way - use direct numeric values:
let position_char = match position.PosiDirection {
    2 => 'L', // Long position
    3 => 'S', // Short position
    1 => 'N', // Net position
    _ => '?', // Unknown
};
```

Common CTP enum values:
- Position Direction (TThostFtdcPosiDirectionType): 1=Net, 2=Long, 3=Short
- Direction (TThostFtdcDirectionType): '0'=Buy, '1'=Sell
- Order Status (TThostFtdcOrderStatusType): '0'=AllTraded, '1'=PartTradedQueueing, '2'=PartTradedNotQueueing, '3'=NoTradeQueueing, '4'=NoTradeNotQueueing, '5'=Canceled, 'a'=Unknown, 'b'=NotTouched, 'c'=Touched

### Setting String Values in CTP Structs

When setting string values in CTP structs, you need to handle the conversion from Rust strings to C-style character arrays:

```rust
// Example of setting BrokerID and UserID in a login field
unsafe {
    use std::ptr::copy_nonoverlapping;
    let broker_bytes = broker_id.as_bytes();
    let user_bytes = user_id.as_bytes();
    
    copy_nonoverlapping(
        broker_bytes.as_ptr(), 
        req_user_login.BrokerID.as_mut_ptr() as *mut u8,
        std::cmp::min(broker_bytes.len(), req_user_login.BrokerID.len() - 1)
    );
    
    copy_nonoverlapping(
        user_bytes.as_ptr(), 
        req_user_login.UserID.as_mut_ptr() as *mut u8,
        std::cmp::min(user_bytes.len(), req_user_login.UserID.len() - 1)
    );
}
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
- Author of the project [ctp-rs](https://github.com/WiSaGaN/ctp-rs). (This project is the cornerstone of CTP migrations in the world of RustLang helped a lot of people. It is archived and read only, hopefully the author is rich enough to retire) 
---

*Note: This is an unofficial Rust binding. CTP is a trademark of Shanghai Futures Exchange.*
