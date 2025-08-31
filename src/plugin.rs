use std::fmt;
use revm::Database;
use tracing::info;
use serde::{Deserialize, Serialize};

use crate::HelloWorldInspector;

/// Plugin that registers the HelloWorldInspector with reth
#[derive(Debug, Default, Clone)]
pub struct HelloWorldInspectorPlugin {
    config: HelloWorldInspectorConfig,
}

impl fmt::Display for HelloWorldInspectorPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HelloWorldInspectorPlugin")
    }
}

/// Configuration for the HelloWorldInspector plugin
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HelloWorldInspectorConfig {
    /// Enable verbose logging
    pub verbose: bool,
    /// Enable step-by-step execution logging
    pub log_steps: bool,
    /// Enable call tracing
    pub trace_calls: bool,
}

impl HelloWorldInspectorPlugin {
    /// Create a new plugin with the given configuration
    pub fn new(config: HelloWorldInspectorConfig) -> Self {
        Self { config }
    }
    
    /// Get the plugin name
    pub fn name(&self) -> &'static str {
        "hello-world-inspector"
    }
    
    /// Get the plugin configuration
    pub fn config(&self) -> &HelloWorldInspectorConfig {
        &self.config
    }
    
    /// Initialize the plugin
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing HelloWorldInspector plugin with config: {:?}", self.config);
        Ok(())
    }
    
    /// Create an inspector instance
    pub fn create_inspector(&self) -> HelloWorldInspector {
        info!("Creating HelloWorldInspector instance");
        HelloWorldInspector::default()
    }
}

/// Register the HelloWorldInspector with reth's EVM configuration
pub fn register_inspector<DB: Database>() {
    let inspector = HelloWorldInspector::default();
    info!("Registering HelloWorldInspector with reth EVM");
    
    // Note: This is a simplified registration approach
    // In practice, you would integrate this with reth's inspector stack
    // through the EVM configuration or inspector handle registration
}

/// Helper function to create and configure the plugin
pub fn create_plugin() -> HelloWorldInspectorPlugin {
    HelloWorldInspectorPlugin::default()
}

/// Helper function to create plugin configuration
pub fn create_config(verbose: bool) -> HelloWorldInspectorConfig {
    HelloWorldInspectorConfig { 
        verbose,
        log_steps: true,
        trace_calls: true,
    }
}

/// Helper function to create plugin configuration with all options
pub fn create_detailed_config(verbose: bool, log_steps: bool, trace_calls: bool) -> HelloWorldInspectorConfig {
    HelloWorldInspectorConfig { 
        verbose,
        log_steps,
        trace_calls,
    }
}