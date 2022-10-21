
cargo install trunk
rustup target add wasm32-unknown-unknown
cargo build
trunk build --release
