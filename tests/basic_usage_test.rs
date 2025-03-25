use std::ffi::CString;
use std::time::Duration;
use std::thread;

// Import the RTP trader API
use rtp::trader::{GenericTraderApi, TraderApi, TraderSpi, ResumeType};
use rtp::trader::{DisconnectionReason, RspResult};
use rtp::trader::CThostFtdcRspUserLoginField;
use rtp::binding::TThostFtdcRequestIDType;

// A simple implementation of TraderSpi to handle callbacks
struct TestTraderSpi {
    connected: bool,
    disconnected: bool,
    login_received: bool,
}

impl TestTraderSpi {
    fn new() -> Self {
        TestTraderSpi {
            connected: false,
            disconnected: false,
            login_received: false,
        }
    }
}

impl TraderSpi for TestTraderSpi {
    fn on_front_connected(&mut self) {
        println!("Connected to trading server");
        self.connected = true;
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("Disconnected from trading server: {:?}", reason);
        self.disconnected = true;
    }

    fn on_rsp_user_login(&mut self, 
                         rsp_user_login: Option<&CThostFtdcRspUserLoginField>, 
                         result: RspResult, 
                         request_id: TThostFtdcRequestIDType, 
                         is_last: bool) {
        println!("Login response: {:?}, result: {:?}, request_id: {}, is_last: {}", 
                 rsp_user_login, result, request_id, is_last);
        self.login_received = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trader_api_basic_usage() {
        // Create a new instance of TraderApi with a flow path
        let flow_path = CString::new("./flow_path").unwrap();
        let mut trader = TraderApi::new(flow_path);
        
        // Get and print API version
        let version = trader.version();
        println!("RTP API Version: {}", version);
        
        // Register a trader spi to handle callbacks
        let trader_spi = Box::new(TestTraderSpi::new());
        trader.register_spi(trader_spi);
        
        // Configure the trader API
        trader.subscribe_public_topic(ResumeType::Quick);
        trader.subscribe_private_topic(ResumeType::Quick);
        
        // Initialize API (non-blocking call)
        trader.init();
        
        // Note: In a real application, you would:
        // 1. Register front address with trader.register_front(...)
        // 2. Authenticate with trader.req_authenticate(...)
        // 3. Login with trader.req_user_login(...)
        // 4. Handle callbacks in the TraderSpi implementation
        
        // For this test, just print that we've initialized successfully
        println!("Trader API initialized successfully");
        
        // Allow some time for potential callbacks
        thread::sleep(Duration::from_millis(100));
        
        // No actual connection in this example
        // In real usage, you would wait for on_front_connected callback
        // and proceed with authentication and login
    }
} 