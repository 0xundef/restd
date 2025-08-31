// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

/// @title HelloWorldTest
/// @notice A test contract for demonstrating HelloWorldInspector functionality
/// @dev This contract includes various operations to showcase inspector capabilities
contract HelloWorldTest {
    // State variables
    uint256 public counter;
    string public message;
    mapping(address => uint256) public balances;
    address public owner;
    
    // Events
    event CounterIncremented(uint256 newValue);
    event MessageUpdated(string newMessage);
    event BalanceUpdated(address indexed user, uint256 newBalance);
    event HelloWorldCalled(address indexed caller, string message);
    
    // Custom errors
    error OnlyOwner();
    error InvalidAmount();
    
    constructor() {
        owner = msg.sender;
        message = "Hello, World!";
        counter = 0;
    }
    
    modifier onlyOwner() {
        if (msg.sender != owner) revert OnlyOwner();
        _;
    }
    
    /// @notice Simple function that emits an event
    function sayHello() external {
        emit HelloWorldCalled(msg.sender, "Hello from Solidity!");
    }
    
    /// @notice Increment the counter and emit an event
    function incrementCounter() external {
        counter++;
        emit CounterIncremented(counter);
    }
    
    /// @notice Update the stored message
    /// @param newMessage The new message to store
    function updateMessage(string calldata newMessage) external {
        message = newMessage;
        emit MessageUpdated(newMessage);
    }
    
    /// @notice Add balance for a user
    /// @param user The user address
    /// @param amount The amount to add
    function addBalance(address user, uint256 amount) external {
        if (amount == 0) revert InvalidAmount();
        balances[user] += amount;
        emit BalanceUpdated(user, balances[user]);
    }
    
    /// @notice Complex function with multiple operations
    /// @param iterations Number of iterations to perform
    function complexOperation(uint256 iterations) external {
        for (uint256 i = 0; i < iterations; i++) {
            counter++;
            balances[msg.sender] += i;
            
            if (i % 2 == 0) {
                emit CounterIncremented(counter);
            }
        }
        
        message = string(abi.encodePacked("Completed ", _toString(iterations), " iterations"));
        emit MessageUpdated(message);
    }
    
    /// @notice Function that reverts to test error handling
    function forceRevert() external pure {
        revert("This function always reverts");
    }
    
    /// @notice Function with nested calls
    function nestedCalls() external {
        _internalFunction1();
        _internalFunction2();
    }
    
    /// @notice Reset all state (only owner)
    function reset() external onlyOwner {
        counter = 0;
        message = "Hello, World!";
        emit MessageUpdated(message);
        emit CounterIncremented(counter);
    }
    
    // Internal functions
    function _internalFunction1() internal {
        counter += 10;
        emit CounterIncremented(counter);
    }
    
    function _internalFunction2() internal {
        balances[msg.sender] += 100;
        emit BalanceUpdated(msg.sender, balances[msg.sender]);
    }
    
    /// @notice Convert uint256 to string
    function _toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) {
            return "0";
        }
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }
    
    // View functions
    function getBalance(address user) external view returns (uint256) {
        return balances[user];
    }
    
    function getContractInfo() external view returns (uint256, string memory, address) {
        return (counter, message, owner);
    }
}