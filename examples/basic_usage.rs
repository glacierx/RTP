use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Update imports to use only public modules and types
use rtp::trader::{GenericTraderApi, TraderApi, TraderSpi, ResumeType};
use rtp::trader::{DisconnectionReason, RspResult};
// Import CThostFtdc types from trader module instead of directly from binding
use rtp::trader::{
    CThostFtdcRspAuthenticateField, CThostFtdcRspUserLoginField, CThostFtdcUserLogoutField,
    CThostFtdcTradingAccountField, CThostFtdcInvestorPositionField, TThostFtdcRequestIDType,
    CThostFtdcReqUserLoginField, CThostFtdcQryTradingAccountField, CThostFtdcQryInvestorPositionField,
};

// A complete trader SPI implementation for basic usage demonstration
struct DemoTraderSpi {
    // Connection state
    connected: Arc<Mutex<bool>>,
    authenticated: Arc<Mutex<bool>>,
    logged_in: Arc<Mutex<bool>>,
    login_failed: Arc<Mutex<bool>>,
    
    // Trading account information
    account_info: Arc<Mutex<Option<CThostFtdcTradingAccountField>>>,
    
    // Position information
    positions: Arc<Mutex<Vec<CThostFtdcInvestorPositionField>>>,
    
    // Request IDs for tracking different requests
    request_id: Arc<Mutex<i32>>,
}

impl DemoTraderSpi {
    fn new(_broker_id: &str, _investor_id: &str, _password: &str) -> Self {
        DemoTraderSpi {
            connected: Arc::new(Mutex::new(false)),
            authenticated: Arc::new(Mutex::new(false)),
            logged_in: Arc::new(Mutex::new(false)),
            login_failed: Arc::new(Mutex::new(false)),
            account_info: Arc::new(Mutex::new(None)),
            positions: Arc::new(Mutex::new(Vec::new())),
            request_id: Arc::new(Mutex::new(0)),
        }
    }
}

impl TraderSpi for DemoTraderSpi {
    fn on_front_connected(&mut self) {
        println!("Connected to trading server. Starting authentication...");
        *self.connected.lock().unwrap() = true;
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("Disconnected from trading server: {:?}", reason);
        *self.connected.lock().unwrap() = false;
        *self.authenticated.lock().unwrap() = false;
        *self.logged_in.lock().unwrap() = false;
    }
    
    fn on_rsp_authenticate(
        &mut self,
        _rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>,
        result: RspResult,
        _request_id: TThostFtdcRequestIDType,
        _is_last: bool,
    ) {
        match result {
            Ok(()) => {
                println!("Authentication successful. Proceeding to login...");
                *self.authenticated.lock().unwrap() = true;
            },
            Err(e) => {
                println!("Authentication failed: [{0}] {1}", e.id, e.msg);
                *self.authenticated.lock().unwrap() = false;
            }
        }
    }
    
    fn on_rsp_user_login(
        &mut self,
        rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        result: RspResult,
        _request_id: TThostFtdcRequestIDType,
        _is_last: bool,
    ) {
        match result {
            Ok(()) => {
                if let Some(login_info) = rsp_user_login {
                    // Print login success information
                    let trading_day = rtp::trader::gb18030_cstr_to_str(&login_info.TradingDay);
                    let login_time = rtp::trader::gb18030_cstr_to_str(&login_info.LoginTime);
                    let system_name = rtp::trader::gb18030_cstr_to_str(&login_info.SystemName);
                    
                    println!("Login successful!");
                    println!("Trading Day: {}", trading_day);
                    println!("Login Time: {}", login_time);
                    println!("System Name: {}", system_name);
                    println!("Front ID: {}", login_info.FrontID);
                    println!("Session ID: {}", login_info.SessionID);
                    
                    *self.logged_in.lock().unwrap() = true;
                }
            },
            Err(e) => {
                println!("Login failed: [{0}] {1}", e.id, e.msg);
                *self.login_failed.lock().unwrap() = true;
            }
        }
    }
    
    fn on_rsp_user_logout(
        &mut self,
        _rsp_user_logout: Option<&CThostFtdcUserLogoutField>,
        _result: RspResult,
        _request_id: TThostFtdcRequestIDType,
        _is_last: bool,
    ) {
        println!("Logged out from trading server.");
        *self.logged_in.lock().unwrap() = false;
    }
    
    fn on_rsp_qry_trading_account(
        &mut self,
        trading_account: Option<&CThostFtdcTradingAccountField>,
        result: RspResult,
        _request_id: TThostFtdcRequestIDType,
        _is_last: bool,
    ) {
        match result {
            Ok(()) => {
                if let Some(account) = trading_account {
                    // Store account information
                    *self.account_info.lock().unwrap() = Some(*account);
                    
                    // Print account details
                    let account_id = rtp::trader::gb18030_cstr_to_str(&account.AccountID);
                    println!("\n-- Account Information --");
                    println!("Account ID: {}", account_id);
                    println!("Balance: {:.2}", account.Balance);
                    println!("Available: {:.2}", account.Available);
                    println!("Frozen Margin: {:.2}", account.FrozenMargin);
                    println!("Commission: {:.2}", account.Commission);
                    println!("Frozen Commission: {:.2}", account.FrozenCommission);
                    println!("Profit: {:.2}", account.CloseProfit + account.PositionProfit);
                    println!("-------------------------\n");
                }
            },
            Err(e) => {
                println!("Failed to query trading account: [{0}] {1}", e.id, e.msg);
            }
        }
    }
    
    fn on_rsp_qry_investor_position(
        &mut self,
        investor_position: Option<&CThostFtdcInvestorPositionField>,
        result: RspResult,
        _request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        match result {
            Ok(()) => {
                if let Some(position) = investor_position {
                    // Store the position
                    let mut positions = self.positions.lock().unwrap();
                    positions.push(*position);
                    
                    // Print position information
                    let instrument_id = rtp::trader::gb18030_cstr_to_str(&position.InstrumentID);
                    
                    // Get position direction as a character
                    // In CTP: 2=Long, 3=Short, 1=Net
                    let position_char = match position.PosiDirection {
                        2 => 'L',
                        3 => 'S',
                        1 => 'N',
                        _ => '?',
                    };
                    
                    println!("Position: {} ({}) - Volume: {}, Cost: {:.2}", 
                             instrument_id,
                             position_char,
                             position.Position,
                             position.OpenCost);
                }
                
                if is_last {
                    println!("\nAll positions received. Total positions: {}", 
                             self.positions.lock().unwrap().len());
                }
            },
            Err(e) => {
                println!("Failed to query investor position: [{0}] {1}", e.id, e.msg);
            }
        }
    }
    
    // Additional callback handlers would go here for order insertion, execution, etc.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration parameters
    let flow_path = CString::new("./flow_path")
        .expect("Failed to create flow path string");
    
    // Trading server details - replace with actual values
    let front_address = "tcp://180.168.146.187:10130"; // Example SimNow address
    let broker_id = "9999";                            // Example SimNow broker ID
    let investor_id = "your_investor_id";              // Replace with your ID
    let password = "your_password";                    // Replace with your password
    
    println!("RTP Basic Usage Example");
    println!("=======================");
    
    // Create TraderApi instance
    let mut trader = TraderApi::new(flow_path);
    println!("API Version: {}", trader.version());
    
    // Create and register our trader SPI
    let trader_spi = Box::new(DemoTraderSpi::new(broker_id, investor_id, password));
    
    // Keep references for checking state
    let connected_ref = trader_spi.connected.clone();
    let logged_in_ref = trader_spi.logged_in.clone();
    let login_failed_ref = trader_spi.login_failed.clone();
    let req_id_ref = trader_spi.request_id.clone();
    
    // Register the SPI with the trader API
    trader.register_spi(trader_spi);
    
    // Configure the trader
    trader.subscribe_public_topic(ResumeType::Resume);
    trader.subscribe_private_topic(ResumeType::Resume);
    
    // Register front address
    trader.register_front(CString::new(front_address)
        .expect("Failed to create front address string"));
    
    // Initialize the API (non-blocking)
    println!("Initializing trader API...");
    trader.init();
    
    // Wait for connection
    println!("Waiting for connection...");
    let mut timeout = 10; // seconds
    while !*connected_ref.lock().unwrap() && timeout > 0 {
        thread::sleep(Duration::from_secs(1));
        timeout -= 1;
    }
    
    if !*connected_ref.lock().unwrap() {
        println!("Failed to connect to trading server within timeout period.");
        return Ok(());
    }
    
    // Authenticate and login
    println!("Sending login request...");
    
    // In a real application, you would authenticate first, but we'll skip to login for simplicity
    let mut req_user_login = CThostFtdcReqUserLoginField::default();
    
    // Fill in login fields
    unsafe {
        use std::ptr::copy_nonoverlapping;
        let broker_bytes = broker_id.as_bytes();
        let investor_bytes = investor_id.as_bytes();
        let password_bytes = password.as_bytes();
        
        copy_nonoverlapping(broker_bytes.as_ptr(), req_user_login.BrokerID.as_mut_ptr() as *mut u8, 
                            std::cmp::min(broker_bytes.len(), req_user_login.BrokerID.len() - 1));
        
        copy_nonoverlapping(investor_bytes.as_ptr(), req_user_login.UserID.as_mut_ptr() as *mut u8,
                            std::cmp::min(investor_bytes.len(), req_user_login.UserID.len() - 1));
        
        copy_nonoverlapping(password_bytes.as_ptr(), req_user_login.Password.as_mut_ptr() as *mut u8,
                            std::cmp::min(password_bytes.len(), req_user_login.Password.len() - 1));
    }
    
    // Get next request ID
    let request_id = {
        let mut id = req_id_ref.lock().unwrap();
        *id += 1;
        *id
    };
    
    // Send login request
    match trader.req_user_login(&req_user_login, request_id) {
        Ok(_) => println!("Login request sent successfully."),
        Err(e) => println!("Failed to send login request: {:?}", e),
    }
    
    // Wait for login response
    println!("Waiting for login response...");
    timeout = 10; // seconds
    while !*logged_in_ref.lock().unwrap() && !*login_failed_ref.lock().unwrap() && timeout > 0 {
        thread::sleep(Duration::from_secs(1));
        timeout -= 1;
    }
    
    if !*logged_in_ref.lock().unwrap() {
        println!("Login failed or timed out. Exiting...");
        return Ok(());
    }
    
    println!("Successfully logged in!");
    
    // Query account information
    println!("\nQuerying account information...");
    let mut qry_trading_account = CThostFtdcQryTradingAccountField::default();
    
    // Fill broker ID
    unsafe {
        let broker_bytes = broker_id.as_bytes();
        std::ptr::copy_nonoverlapping(
            broker_bytes.as_ptr(),
            qry_trading_account.BrokerID.as_mut_ptr() as *mut u8,
            std::cmp::min(broker_bytes.len(), qry_trading_account.BrokerID.len() - 1)
        );
    }
    
    // Send account query
    let request_id = {
        let mut id = req_id_ref.lock().unwrap();
        *id += 1;
        *id
    };
    
    match trader.req_qry_trading_account(&qry_trading_account, request_id) {
        Ok(_) => println!("Account query request sent successfully."),
        Err(e) => println!("Failed to send account query: {:?}", e),
    }
    
    // Wait a moment for the response
    thread::sleep(Duration::from_secs(2));
    
    // Query positions
    println!("\nQuerying positions...");
    let mut qry_investor_position = CThostFtdcQryInvestorPositionField::default();
    
    // Fill broker ID and investor ID
    unsafe {
        let broker_bytes = broker_id.as_bytes();
        let investor_bytes = investor_id.as_bytes();
        
        std::ptr::copy_nonoverlapping(
            broker_bytes.as_ptr(),
            qry_investor_position.BrokerID.as_mut_ptr() as *mut u8,
            std::cmp::min(broker_bytes.len(), qry_investor_position.BrokerID.len() - 1)
        );
        
        std::ptr::copy_nonoverlapping(
            investor_bytes.as_ptr(),
            qry_investor_position.InvestorID.as_mut_ptr() as *mut u8,
            std::cmp::min(investor_bytes.len(), qry_investor_position.InvestorID.len() - 1)
        );
    }
    
    // Send position query
    let request_id = {
        let mut id = req_id_ref.lock().unwrap();
        *id += 1;
        *id
    };
    
    match trader.req_qry_investor_position(&qry_investor_position, request_id) {
        Ok(_) => println!("Position query request sent successfully."),
        Err(e) => println!("Failed to send position query: {:?}", e),
    }
    
    // Wait for responses to be processed
    println!("\nWaiting for query responses...");
    thread::sleep(Duration::from_secs(5));
    
    // Logout
    println!("\nLogging out...");
    let mut req_user_logout = CThostFtdcUserLogoutField::default();
    
    // Fill broker ID and investor ID
    unsafe {
        let broker_bytes = broker_id.as_bytes();
        let investor_bytes = investor_id.as_bytes();
        
        std::ptr::copy_nonoverlapping(
            broker_bytes.as_ptr(),
            req_user_logout.BrokerID.as_mut_ptr() as *mut u8,
            std::cmp::min(broker_bytes.len(), req_user_logout.BrokerID.len() - 1)
        );
        
        std::ptr::copy_nonoverlapping(
            investor_bytes.as_ptr(),
            req_user_logout.UserID.as_mut_ptr() as *mut u8,
            std::cmp::min(investor_bytes.len(), req_user_logout.UserID.len() - 1)
        );
    }
    
    // Send logout request
    let request_id = {
        let mut id = req_id_ref.lock().unwrap();
        *id += 1;
        *id
    };
    
    match trader.req_user_logout(&req_user_logout, request_id) {
        Ok(_) => println!("Logout request sent successfully."),
        Err(e) => println!("Failed to send logout request: {:?}", e),
    }
    
    // Wait for logout to complete
    thread::sleep(Duration::from_secs(2));
    
    println!("\nExample completed successfully.");
    Ok(())
}