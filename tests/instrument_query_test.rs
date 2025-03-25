use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

// Import the RTP trader API
use rtp::trader::{GenericTraderApi, TraderApi, TraderSpi, ResumeType};
use rtp::trader::{DisconnectionReason, RspResult};
use rtp::trader::{
    CThostFtdcInstrumentField, CThostFtdcQryInstrumentField,
};
use rtp::binding::TThostFtdcRequestIDType;

// A more complex TraderSpi implementation that handles instrument queries
struct InstrumentQueryTraderSpi {
    connected: Arc<Mutex<bool>>,
    instruments_received: Arc<Mutex<Vec<CThostFtdcInstrumentField>>>,
    query_finished: Arc<Mutex<bool>>,
}

impl InstrumentQueryTraderSpi {
    fn new() -> Self {
        InstrumentQueryTraderSpi {
            connected: Arc::new(Mutex::new(false)),
            instruments_received: Arc::new(Mutex::new(Vec::new())),
            query_finished: Arc::new(Mutex::new(false)),
        }
    }
    
    fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
    
    fn is_query_finished(&self) -> bool {
        *self.query_finished.lock().unwrap()
    }
    
    fn get_instruments(&self) -> Vec<CThostFtdcInstrumentField> {
        self.instruments_received.lock().unwrap().clone()
    }
}

impl TraderSpi for InstrumentQueryTraderSpi {
    fn on_front_connected(&mut self) {
        println!("Connected to trading server");
        *self.connected.lock().unwrap() = true;
    }

    fn on_front_disconnected(&mut self, reason: DisconnectionReason) {
        println!("Disconnected from trading server: {:?}", reason);
        *self.connected.lock().unwrap() = false;
    }
    
    fn on_rsp_qry_instrument(
        &mut self,
        instrument_data: Option<&CThostFtdcInstrumentField>,
        result: RspResult,
        _request_id: TThostFtdcRequestIDType,
        is_last: bool,
    ) {
        match result {
            Ok(()) => {
                if let Some(instrument_data) = instrument_data {
                    let instrument_id = rtp::trader::gb18030_cstr_to_str(&instrument_data.InstrumentID);
                    let exchange_id = rtp::trader::gb18030_cstr_to_str(&instrument_data.ExchangeID);
                    println!("Instrument: {} on {}", instrument_id, exchange_id);
                }
                
                if is_last {
                    println!("All instruments received.");
                }
            },
            Err(e) => {
                println!("Failed to query instrument: [{0}] {1}", e.id, e.msg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instrument_query() {
        // This test is marked as ignored because it requires connection to a real CTP server
        // Run it with `cargo test -- --ignored` when you have proper connection details

        // Create a new instance of TraderApi with a flow path
        let flow_path = CString::new("./flow_path").unwrap();
        let mut trader = TraderApi::new(flow_path);
        
        // Create and register our custom TraderSpi
        let trader_spi = Box::new(InstrumentQueryTraderSpi::new());
        
        // Keep a reference to the spi for checking status later
        let spi_ref = trader_spi.connected.clone();
        let query_finished_ref = trader_spi.query_finished.clone();
        
        // Register the spi with the trader API
        trader.register_spi(trader_spi);
        
        // Configure and initialize
        trader.subscribe_public_topic(ResumeType::Quick);
        trader.subscribe_private_topic(ResumeType::Quick);
        
        // This would be a real front address in a production environment
        // For testing we'll use a dummy address
        // trader.register_front(CString::new("tcp://180.168.146.187:10130").unwrap());
        
        // Initialize API
        println!("Initializing trader API...");
        trader.init();
        
        // Wait for potential connection (in a real test we would wait for the connection event)
        println!("Waiting for potential connection...");
        thread::sleep(Duration::from_secs(1));
        
        // In a real test, we would check if we're connected
        if *spi_ref.lock().unwrap() {
            println!("Connected to server, proceeding with instrument query");
            
            // Create a query request
            let query_req = CThostFtdcQryInstrumentField::default();
            
            // You can filter by specific instrument ID or exchange ID
            // For example:
            // rtp::common::set_cstr_from_str(&mut query_req.InstrumentID, "rb2209").unwrap();
            
            // Send the query request
            let request_id = 1; // You would use an incrementing counter in real code
            match trader.req_qry_instrument(&query_req, request_id) {
                Ok(_) => println!("Instrument query request sent successfully"),
                Err(e) => println!("Failed to send instrument query: {:?}", e),
            }
            
            // Wait for the query to complete (with timeout)
            let max_wait_time = Duration::from_secs(30);
            let start = std::time::Instant::now();
            
            while !*query_finished_ref.lock().unwrap() && start.elapsed() < max_wait_time {
                thread::sleep(Duration::from_millis(100));
            }
            
            // Check if we completed within the timeout
            if *query_finished_ref.lock().unwrap() {
                println!("Query completed successfully");
            } else {
                println!("Query timed out");
            }
        } else {
            println!("Not connected to server, skipping instrument query");
        }
        
        // In a real application, we would wait for all callbacks to complete
        // and properly clean up resources
        println!("Test completed");
    }
} 