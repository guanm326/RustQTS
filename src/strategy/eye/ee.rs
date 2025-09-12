use crate::strategy::eye::params::EEConfig;
use chrono;
use std::time::Duration;
use tokio;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ElectronicEye {
    params: EEConfig,




    // Class variables
    last_poll_time: i64,
}


impl ElectronicEye {
    pub fn new(params: EEConfig) -> Self {
        // Initialize the ElectronicEye constructor
        let mut ee = ElectronicEye {
            params,
            last_poll_time: 0
        };
        
        // Call Initalization methods
        ee.init_exchanges();
        ee // return the ElectronicEye instance
    }
    
    fn init_exchanges(&mut self) {

        println!("init_exchanges....");



        
    }

    pub async fn process_book_updates(&mut self) {


        println!("process_book_updates....");

        // Process book updates
        tokio::time::sleep(Duration::from_secs(1)).await;




    }

    pub async fn process_fills(&mut self) {


        // Process fills
        println!("process_fills....");

        //Sleep for 1 second
        tokio::time::sleep(Duration::from_secs(1)).await;


    }


    pub async fn process_backoffice(&mut self) {

        // Process backoffice   
        println!("process_backoffice....");

        // Get current time
        // Check if its poll_interval_seconds, if so, poll
        let current_time = chrono::Utc::now().timestamp_millis();
        if current_time - self.last_poll_time > (self.params.quote_params.poll_interval_seconds as i64) * 1000 {
            self.poll().await;
            self.last_poll_time = current_time;
        }
        
        tokio::time::sleep(Duration::from_secs(1)).await;
    


    }


    pub async fn process_config_updates(&mut self) {
        // Process config updates
    }


    pub async fn poll(&self) {
        // Add your hedge logic here


        println!("Polled....");

    }
    
    pub async fn run(mut self) {
        // Query Pool during initialization
        self.poll().await;
        self.last_poll_time = chrono::Utc::now().timestamp_millis();

        loop {


            self.process_book_updates().await;
            self.process_fills().await;
            self.process_backoffice().await;




        }
    }
}


