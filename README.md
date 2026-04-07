## Overview
Plantplot 

## Tech stack
This project is built with a focus on performance, observability, and modern backend practices:
- Rust - Used for its zero-cost abstractions, strong type system, and fearless concurrency
- Axum - Built on top of tokio and hyper, enabling scalable and composable web services
- Tracing - Provides structured, contextual logging designed for async Rust applications

## Installation
```
git clone https://github.com/devmichalek/plantplot.git
cd plantplot
cargo build --release
```

## Usage
To run plantplot:
```
cargo run --release --bin webserver & &> /dev/null
cargo run --release --bin gui
```
To debug plantplot's component:
```
RUST_LOG=debug cargo run --debug --bin webserver
```