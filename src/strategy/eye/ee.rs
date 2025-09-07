use crate::strategy::eye::params::EEConfig;
use chrono;

pub struct ElectronicEye {
    params: EEConfig,
    // Class variables
    last_poll_time: i64,
}


impl ElectronicEye {
    pub fn new(params: EEConfig) -> Self {
        ElectronicEye {
            params,
            last_poll_time: 0
        }
    }


    pub async fn run(&mut self) {
        // Query Pool during initialization
        self.poll().await;
        self.last_poll_time = chrono::Utc::now().timestamp_millis();

        loop {
            // Check if its poll_interval_seconds, if so, poll
            // Get current time
            let current_time = chrono::Utc::now().timestamp_millis();
            if current_time - self.last_poll_time > (self.params.quote_params.poll_interval_seconds as i64) * 1000 {
                self.poll().await;
                self.last_poll_time = current_time;
            }
        }
    }

    pub async fn poll(&self) {
        // Add your hedge logic here


        println!("Polled....");

    }
    

}