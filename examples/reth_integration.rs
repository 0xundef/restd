//! Example demonstrating how to integrate HelloWorldInspector with a reth node
//!
//! This example shows how to:
//! 1. Create a HelloWorldInspector plugin
//! 2. Configure the inspector for reth integration
//! 3. Demonstrate plugin initialization and usage

use restd::{
    HelloWorldInspector, 
    HelloWorldInspectorPlugin, 
    create_plugin, 
    create_config,
    create_detailed_config
};
use tokio;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize basic tracing (no external subscriber needed)
    println!("Starting HelloWorldInspector reth integration example...");

    info!("Starting reth node with HelloWorldInspector plugin");

    // Create the plugin and configuration
    let plugin = create_plugin();
    let config = create_config(true); // Enable verbose logging

    info!("Plugin created: {}", plugin.name());
    info!("Plugin config - verbose: {}", config.verbose);

    // Example of how you would integrate with a reth node
    // Note: This is a simplified example. In practice, you would:
    // 1. Create a custom node type that includes your inspector
    // 2. Register the plugin with reth's plugin system
    // 3. Configure the EVM to use your inspector

    println!("\n=== HelloWorldInspector Reth Integration Example ===");
    println!("Plugin Name: {}", plugin.name());
    println!("Verbose Mode: {}", config.verbose);
    println!("\nTo integrate with reth:");
    println!("1. Add this crate as a dependency to your reth node");
    println!("2. Register the plugin in your node's plugin configuration");
    println!("3. Configure the EVM to use the HelloWorldInspector");
    println!("\nExample usage in reth node configuration:");
    println!("```rust");
    println!("use restd::{{HelloWorldInspectorPlugin, create_config}};");
    println!("\nlet plugin = HelloWorldInspectorPlugin::default();");
    println!("let config = create_config(true);");
    println!("// Register with reth's plugin system");
    println!("```");

    Ok(())
}

/// Example function showing how to create a custom EVM configuration
/// that includes the HelloWorldInspector
pub fn create_evm_with_inspector() {
    info!("Creating EVM configuration with HelloWorldInspector");
    
    let inspector = HelloWorldInspector::default();
    
    // In a real reth integration, you would:
    // 1. Create an EVM configuration
    // 2. Add the inspector to the inspector stack
    // 3. Configure the EVM to use this stack
    
    info!("Inspector created and ready for EVM integration");
}

/// Example function showing how to register the plugin with reth
pub async fn register_with_reth() -> Result<(), Box<dyn std::error::Error>> {
    info!("Registering HelloWorldInspector plugin with reth");
    
    let plugin = create_plugin();
    let config = create_config(false);
    
    // In a real implementation, you would:
    // 1. Get the reth node configuration
    // 2. Initialize the plugin with the node config
    // 3. Register the plugin with reth's plugin manager
    
    info!("Plugin {} registered successfully", plugin.name());
    
    Ok(())
}