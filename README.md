# HelloWorldInspector Reth Plugin Integration

This document explains how to integrate the `HelloWorldInspector` with a reth node as a plugin.

## Overview

The `HelloWorldInspector` can be loaded into reth as a plugin to monitor EVM execution events. This integration allows you to:

- Track EVM step execution
- Monitor contract calls and creation
- Log EVM events and state changes
- Debug transaction execution

## Installation

### 1. Add as Dependency

Add this crate to your reth node's `Cargo.toml`:

```toml
[dependencies]
restd = { path = "path/to/restd" }
# Or from git:
# restd = { git = "https://github.com/your-repo/restd.git" }
```

### 2. Import the Plugin

```rust
use restd::{
    HelloWorldInspectorPlugin,
    HelloWorldInspectorConfig,
    create_plugin,
    create_config,
};
```

## Usage

### Basic Plugin Setup

```rust
use restd::{create_plugin, create_config};

// Create the plugin
let plugin = create_plugin();
let config = create_config(true); // Enable verbose logging

// The plugin name is "hello-world-inspector"
println!("Plugin: {}", plugin.name());
```

### Integration with Reth Node

#### Method 1: Plugin Registration

```rust
use reth_node_api::Plugin;
use restd::HelloWorldInspectorPlugin;

// In your reth node setup
let plugin = HelloWorldInspectorPlugin::default();
let config = HelloWorldInspectorConfig { verbose: true };

// Register with reth's plugin system
// (Implementation depends on your specific reth node setup)
```

#### Method 2: EVM Configuration

```rust
use restd::{HelloWorldInspector, register_inspector};
use reth_evm::ConfigureEvm;

// Register the inspector with reth's EVM
register_inspector::<YourDatabase>();

// Or manually create and configure
let inspector = HelloWorldInspector::default();
// Add to your EVM configuration's inspector stack
```

#### Method 3: Custom Node Implementation

```rust
use reth_node_core::node_config::NodeConfig;
use restd::HelloWorldInspectorPlugin;

// In your custom node implementation
impl<Node> Plugin<Node> for HelloWorldInspectorPlugin
where
    Node: NodeTypesWithEngine,
{
    // Plugin implementation is already provided
}
```

## Configuration Options

### HelloWorldInspectorConfig

```rust
pub struct HelloWorldInspectorConfig {
    /// Enable verbose logging of EVM events
    pub verbose: bool,
}
```

### Creating Configuration

```rust
use restd::create_config;

// Enable verbose logging
let config = create_config(true);

// Disable verbose logging
let config = create_config(false);
```

## Running the Example

```bash
# Run the integration example
cargo run --example reth_integration

# Build the library
cargo build

# Run tests
cargo test
```

## Inspector Capabilities

The `HelloWorldInspector` provides the following EVM monitoring capabilities:

### Execution Tracking
- **Step Execution**: Monitors each EVM instruction step
- **Gas Usage**: Tracks gas consumption during execution
- **Stack Operations**: Observes stack changes

### Call Monitoring
- **Contract Calls**: Logs external and internal contract calls
- **Call Results**: Tracks call success/failure and return data
- **Call Context**: Monitors caller, callee, and call value

### Contract Creation
- **Creation Events**: Logs contract creation attempts
- **Creation Results**: Tracks creation success/failure
- **Deployed Code**: Monitors deployed bytecode

### Event Logging
- **Log Events**: Captures emitted log events
- **Event Data**: Logs event topics and data
- **Event Context**: Tracks event source contracts

### State Changes
- **Self-Destruct**: Monitors contract self-destruction
- **State Access**: Tracks state read/write operations

## Advanced Usage

### Custom Inspector Implementation

You can extend the `HelloWorldInspector` or create your own:

```rust
use revm::{Inspector, Database, EvmContext};
use restd::HelloWorldInspector;

#[derive(Debug, Default)]
struct CustomInspector {
    base: HelloWorldInspector,
    // Add your custom fields
}

impl<DB: Database> Inspector<DB> for CustomInspector {
    // Implement custom inspection logic
    fn step(&mut self, interp: &mut Interpreter, context: &mut EvmContext<DB>) {
        // Call base inspector
        self.base.step(interp, context);
        
        // Add your custom logic
        println!("Custom step logic");
    }
    
    // Implement other methods as needed
}
```

### Integration with Reth CLI

To integrate with reth's command-line interface:

1. Create a custom reth binary that includes your plugin
2. Register the plugin in the node's plugin configuration
3. Add CLI flags for plugin configuration

```rust
// In your custom reth binary
use reth::cli::Cli;
use restd::HelloWorldInspectorPlugin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize your custom reth node with the plugin
    let cli = Cli::parse();
    
    // Add plugin registration logic here
    
    cli.run().await
}
```

## Troubleshooting

### Common Issues

1. **Plugin Not Loading**: Ensure the plugin is properly registered with reth's plugin system
2. **Inspector Not Called**: Verify the inspector is added to the EVM's inspector stack
3. **Compilation Errors**: Check that reth dependencies match your reth version

### Debug Logging

Enable debug logging to see inspector activity:

```bash
RUST_LOG=debug cargo run --example reth_integration
```