//! Foundry Integration Example for HelloWorldInspector
//!
//! This example demonstrates how to use HelloWorldInspector with revm
//! for contract testing and analysis in a Foundry-like environment.

use alloy_primitives::{Address, U256, Bytes};
use revm::{
    primitives::{TxKind, Env, TxEnv, ResultAndState},
    Evm, InMemoryDB,
};
use restd::{HelloWorldInspector, HelloWorldInspectorConfig};

/// Simple integration example showing HelloWorldInspector usage
struct SimpleIntegration {
    db: InMemoryDB,
    inspector: HelloWorldInspector,
}

impl SimpleIntegration {
    /// Create a new integration instance
    pub fn new(_config: HelloWorldInspectorConfig) -> Self {
        Self {
            db: InMemoryDB::default(),
            inspector: HelloWorldInspector::new(),
        }
    }
    
    /// Execute a simple transaction and return the result
    pub fn execute_transaction(&mut self, caller: Address, to: Option<Address>, data: Bytes) -> Result<ResultAndState, String> {
        // Create EVM environment
        let mut env = Env::default();
        
        // Set up transaction
        env.tx = TxEnv {
            caller,
            gas_limit: 1_000_000,
            gas_price: U256::from(20_000_000_000u64),
            transact_to: match to {
                Some(addr) => TxKind::Call(addr),
                None => TxKind::Create,
            },
            value: U256::ZERO,
            data,
            nonce: Some(0),
            chain_id: Some(1),
            access_list: Vec::new(),
            gas_priority_fee: Some(U256::from(1_000_000_000u64)),
            blob_hashes: Vec::new(),
            max_fee_per_blob_gas: None,
            authorization_list: None,
        };
        
        // Create EVM with inspector
        let mut evm = Evm::builder()
            .with_db(&mut self.db)
            .with_env(Box::new(env))
            .with_external_context(&mut self.inspector)
            .build();
        
        // Execute transaction
        evm.transact().map_err(|e| format!("Transaction failed: {:?}", e))
    }
    
    /// Get inspector statistics
    pub fn get_stats(&self) -> (u64, u64) {
        (self.inspector.step_count, self.inspector.call_count)
    }
}

fn main() {
    println!("🚀 HelloWorldInspector Foundry Integration Example");
    println!("=================================================");
    
    // Test different configurations
    let configs = vec![
        ("Full Tracing", HelloWorldInspectorConfig {
            trace_calls: true,
            log_steps: true,
            verbose: true,
        }),
        ("Call Tracing Only", HelloWorldInspectorConfig {
            trace_calls: true,
            log_steps: false,
            verbose: false,
        }),
        ("Step Logging Only", HelloWorldInspectorConfig {
            trace_calls: false,
            log_steps: true,
            verbose: false,
        }),
    ];
    
    for (name, config) in configs {
        println!("\n📋 Testing configuration: {}", name);
        println!("   trace_calls: {}, log_steps: {}, verbose: {}", 
                 config.trace_calls, config.log_steps, config.verbose);
        
        let mut integration = SimpleIntegration::new(config);
        
        // Execute a simple contract creation
        let caller = Address::from([0x1; 20]);
        let bytecode = Bytes::from(vec![0x60, 0x00, 0x60, 0x00, 0xf3]); // Simple contract bytecode
        
        match integration.execute_transaction(caller, None, bytecode) {
            Ok(_result) => {
                println!("   ✅ Transaction executed successfully");
                let (steps, calls) = integration.get_stats();
                println!("   📊 Inspector stats - Steps: {}, Calls: {}", steps, calls);
            }
            Err(e) => {
                println!("   ❌ Transaction failed: {}", e);
            }
        }
    }
    
    println!("\n🎉 Foundry integration example completed!");
    println!("\n💡 Next steps:");
    println!("   1. Integrate with your Foundry test suite");
    println!("   2. Use the inspector to analyze contract execution");
    println!("   3. Customize the configuration for your needs");
}