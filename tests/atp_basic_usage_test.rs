// This test specifically targets the ATP implementation
// To run: cargo test --features=atp --no-default-features

use std::ffi::CString;
use std::time::Duration;
use std::thread;

// Import the RTP trader API
use rtp::trader::{GenericTraderApi, TraderApi, TraderSpi, ResumeType};
use rtp::common::{DisconnectionReason, RspResult};
use rtp::binding::*;

// A simple ATP trader SPI implementation
struct AtpTraderSpi {
    connected: bool,
    authenticated: bool,
    login_success: bool,
}

impl AtpTraderSpi {
    fn new() -> Self {
        AtpTraderSpi {
            connected: false,
            authenticated: false,
            login_success: false,
        }
    }
}

impl TraderSpi for AtpTraderSpi {
    fn on_front_connected(&mut self) {
        println!("ATP API: Connected to trading server");
        self.connected = true;
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("ATP API: Disconnected from trading server: {:?}", reason);
        self.connected = false;
    }
    
    fn on_rsp_authenticate(
        &mut self,
        rsp_authenticate: Option<&CThostFtdcRspAuthenticateField>,
        result: RspResult,
        request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        println!("ATP API: Authentication response received");
        if let Ok(()) = result {
            println!("ATP API: Authentication successful");
            self.authenticated = true;
        } else {
            println!("ATP API: Authentication failed: {:?}", result);
        }
    }

    fn on_rsp_user_login(
        &mut self,
        rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        result: RspResult,
        request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        println!("ATP API: Login response received");
        if let Ok(()) = result {
            if let Some(login_field) = rsp_user_login {
                let trading_day = rtp::common::gb18030_cstr_to_str(&login_field.TradingDay);
                println!("ATP API: Login successful. Trading day: {}", trading_day);
                self.login_success = true;
            }
        } else {
            println!("ATP API: Login failed: {:?}", result);
        }
    }
}

// ATP specific test
#[cfg(test)]
#[cfg(feature = "atp")]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Ignore by default as it requires real connection details
    fn test_atp_basic_connection() {
        // Create a flow path for ATP
        let flow_path = CString::new("./flow_path_atp").unwrap();
        let mut trader = TraderApi::new(flow_path);
        
        // Get ATP API version
        let version = trader.version();
        println!("ATP API Version: {}", version);
        
        // Register our ATP trader spi
        let trader_spi = Box::new(AtpTraderSpi::new());
        trader.register_spi(trader_spi);
        
        // Configure the ATP trader
        trader.subscribe_private_topic(ResumeType::Resume);
        trader.subscribe_public_topic(ResumeType::Resume);
        
        // In a real scenario, you would register the front address:
        // trader.register_front(CString::new("tcp://your.atp.server.address:port").unwrap());
        
        // Initialize the API
        trader.init();
        println!("ATP Trader API initialized");
        
        // ATP servers typically require authentication before login
        // In a real application, you would:
        // 1. Wait for the on_front_connected callback
        // 2. Send authentication request with proper credentials:
        /*
        let mut req_authenticate = CThostFtdcReqAuthenticateField::default();
        rtp::common::set_cstr_from_str(&mut req_authenticate.BrokerID, "your_broker_id").unwrap();
        rtp::common::set_cstr_from_str(&mut req_authenticate.UserID, "your_user_id").unwrap();
        rtp::common::set_cstr_from_str(&mut req_authenticate.AuthCode, "your_auth_code").unwrap();
        rtp::common::set_cstr_from_str(&mut req_authenticate.AppID, "your_app_id").unwrap();
        
        match trader.req_authenticate(&req_authenticate, 1) {
            Ok(_) => println!("Authentication request sent"),
            Err(e) => println!("Failed to send authentication request: {:?}", e),
        }
        */
        
        // After successful authentication, you would login:
        /*
        let mut req_user_login = CThostFtdcReqUserLoginField::default();
        rtp::common::set_cstr_from_str(&mut req_user_login.BrokerID, "your_broker_id").unwrap();
        rtp::common::set_cstr_from_str(&mut req_user_login.UserID, "your_user_id").unwrap();
        rtp::common::set_cstr_from_str(&mut req_user_login.Password, "your_password").unwrap();
        
        match trader.req_user_login(&req_user_login, 2) {
            Ok(_) => println!("Login request sent"),
            Err(e) => println!("Failed to send login request: {:?}", e),
        }
        */
        
        // For this test, just wait a moment to ensure initialization completes
        thread::sleep(Duration::from_millis(500));
        
        println!("ATP test completed");
    }
} 