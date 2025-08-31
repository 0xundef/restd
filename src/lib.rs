//! A simple "Hello, World!" inspector for reth EVM execution tracing.
//!
//! This library provides a basic implementation of the reth Inspector trait
//! that prints "Hello, world!" messages during various EVM execution events.

use alloy_primitives::{Address, Log, U256};
use revm::{
    interpreter::{CallInputs, CallOutcome, CreateInputs, CreateOutcome, Interpreter},
    EvmContext, Inspector, Database,
};

pub mod plugin;

/// A simple inspector that prints "Hello, world!" during EVM execution events.
/// 
/// This inspector demonstrates the basic usage of the reth Inspector trait
/// by implementing key hooks that are called during EVM execution.
#[derive(Debug, Default)]
pub struct HelloWorldInspector {
    /// Counter to track the number of steps executed
    pub step_count: u64,
    /// Counter to track the number of calls made
    pub call_count: u64,
}

impl HelloWorldInspector {
    /// Creates a new HelloWorldInspector instance.
    pub fn new() -> Self {
        println!("Hello, world! Inspector initialized.");
        Self::default()
    }
    
    /// Returns the current step count.
    pub fn steps(&self) -> u64 {
        self.step_count
    }
    
    /// Returns the current call count.
    pub fn calls(&self) -> u64 {
        self.call_count
    }
}

impl<DB: Database> Inspector<DB> for HelloWorldInspector {
    /// Called before the interpreter is initialized.
    fn initialize_interp(&mut self, _interp: &mut Interpreter, _context: &mut EvmContext<DB>) {
        println!("Hello, world! Interpreter initializing...");
    }

    /// Called on each step of the interpreter.
    fn step(&mut self, interp: &mut Interpreter, _context: &mut EvmContext<DB>) {
        self.step_count += 1;
        
        // Print hello message every 100 steps to avoid spam
        if self.step_count % 100 == 0 {
            println!(
                "Hello, world! Step #{} - Opcode: {:?}", 
                self.step_count,
                interp.current_opcode()
            );
        }
    }

    /// Called after step when the instruction has been executed.
    fn step_end(&mut self, _interp: &mut Interpreter, _context: &mut EvmContext<DB>) {
        // Optional: could add post-step logic here
    }

    /// Called when a log is emitted.
    fn log(&mut self, _interp: &mut Interpreter, _context: &mut EvmContext<DB>, log: &Log) {
        println!(
            "Hello, world! Log emitted with {} topics and {} bytes of data",
            log.topics().len(),
            log.data.data.len()
        );
    }

    /// Called whenever a call to a contract is about to start.
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        self.call_count += 1;
        println!(
            "Hello, world! Call #{} to address: {:?}",
            self.call_count,
            inputs.target_address
        );
        
        // Return None to continue with normal execution
        None
    }

    /// Called when a call to a contract has concluded.
    fn call_end(
        &mut self,
        _context: &mut EvmContext<DB>,
        _inputs: &CallInputs,
        outcome: CallOutcome,
    ) -> CallOutcome {
        println!(
            "Hello, world! Call ended with success: {}",
            outcome.result.is_ok()
        );
        outcome
    }

    /// Called when a contract is about to be created.
    fn create(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CreateInputs,
    ) -> Option<CreateOutcome> {
        println!(
            "Hello, world! Contract creation with {} bytes of code",
            inputs.init_code.len()
        );
        
        // Return None to continue with normal execution
        None
    }

    /// Called when a contract has been created.
    fn create_end(
        &mut self,
        _context: &mut EvmContext<DB>,
        _inputs: &CreateInputs,
        outcome: CreateOutcome,
    ) -> CreateOutcome {
        println!(
            "Hello, world! Contract creation ended with success: {}",
            outcome.result.is_ok()
        );
        outcome
    }

    /// Called when a contract has been self-destructed.
    fn selfdestruct(&mut self, contract: Address, target: Address, value: U256) {
        println!(
            "Hello, world! Contract {:?} self-destructed, sending {} wei to {:?}",
            contract, value, target
        );
    }
}

// Re-export plugin functionality
pub use plugin::{
    HelloWorldInspectorPlugin, 
    HelloWorldInspectorConfig, 
    create_plugin, 
    create_config,
    create_detailed_config,
    register_inspector
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspector_creation() {
        let inspector = HelloWorldInspector::new();
        assert_eq!(inspector.steps(), 0);
        assert_eq!(inspector.calls(), 0);
    }

    #[test]
    fn test_inspector_default() {
        let inspector = HelloWorldInspector::default();
        assert_eq!(inspector.step_count, 0);
        assert_eq!(inspector.call_count, 0);
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = create_plugin();
        assert_eq!(plugin.name(), "hello-world-inspector");
    }

    #[test]
    fn test_plugin_config() {
        let config = create_config(true);
        assert!(config.verbose);
    }
}