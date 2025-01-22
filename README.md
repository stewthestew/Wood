# Wood 🌲  

A simple and colorful logging crate for Rust.  

## Installation  
Add the following to your `Cargo.toml`:  
```toml
[dependencies]
wood = { git = "https://github.com/stewthestew/Wood.git" }
```

## Usage  
```rust
use wood::Logger;

Logger::info("This is an info message.");
Logger::warn("This is a warning message.");
Logger::error("An error occurred!");
Logger::fatal("A fatal error occurred!");
Logger::success("Operation completed successfully.");
Logger::debug("This is a debug message.");
Logger::extra("ALERT", "\x1b[31m", "This is a custom alert.");
```
