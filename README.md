# Wood 🌲  

A simple and colorful logging crate for Rust.

Really it is just one file with a bunch of functions

## Installation  
Add the following to your `Cargo.toml`:  
```toml
[dependencies]
wood = { git = "https://github.com/stewthestew/Wood.git" }
```

## Usage  
```rust
use wood::Logger;

logger::info("This is an info message.");
logger::warn("This is a warning message.");
logger::error("An error occurred!");
logger::fatal("A fatal error occurred!");
logger::success("Operation completed successfully.");
logger::debug("This is a debug message.");
simple::extra("ALERT", "\x1b[31m", "This is a custom alert."); // This will not provide the current date
```

## TODO
- [ ] Switch from functions to macros
