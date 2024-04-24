# Webchaussette
##### Fast, powerful, and easy-to-set-up WebSocket library

[![Version](https://img.shields.io/crates/v/Webchaussette.svg)](https://crates.io/crates/Webchaussette)
[![Documentation](https://docs.rs/webchaussette/badge.svg)](https://docs.rs/webchaussette)
[![License](https://img.shields.io/crates/l/webchaussette.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/VeroniDeev/webchaussette/workflows/CI/badge.svg)](https://github.com/VeroniDeev/webchaussette/actions)

## Installation

To use this library, simply add it to your `Cargo.toml` :

```toml
[dependencies]
webchaussette = "1.0"
async-trait = "0.1"
tokio = "1"
```

## Example
```rust
use webchaussette::server::{EventHandler, Public, Server, Types};

// Implement the field if you wish
struct Test;

#[async_trait::async_trait]
impl EventHandler for Test {
    // Read incoming user data
    async fn on_message(&self, public: &mut Public) {
        match &public.message {
            Types::String(val) => println!("{}", val),
            Types::Binary(val) => println!("{:?}", val),
        }
    }
    async fn on_close(&self) {
        println!("The user has left");
    }
}

#[tokio::main]
async fn main() {
    let mut server: Server = Server::new("0.0.0.0:8080").await;
    server.set_handler(Box::new(Test));
    server.run().await;
}
```

## Documentation
Documentation is being processed !

## Contribution
Contributions are welcome! Feel free to open issues or send pull requests.

## License
This project is licensed under MIT. See the [LICENSE](LICENSE) file for more details
