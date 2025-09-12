use serde::{Deserialize, Serialize};
use std::fs;
use crate::data_structure::Symbol;
use crate::strategy::strategy_structs::{StrategyParams, HedgeParams, QuoteParams, RunMode};

/*
    Yaml Configuration Paramers For EE strategy
*/



// Configure Structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EEConfig {
    pub strategy: StrategyParams,
    pub quote_instrument: Symbol,
    pub hedge_instrument: Symbol,
    pub hedge_params: HedgeParams,
    pub quote_params: QuoteParams,
    pub run_mode: RunMode,
}



impl EEConfig {
    /// Load configuration from YAML file
    pub fn from_yaml_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: EEConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    /// Load configuration from YAML string
    pub fn from_yaml_str(yaml_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: EEConfig = serde_yaml::from_str(yaml_content)?;
        Ok(config)
    }
}

