sdk use java 11.0.14.9.1-amzn
cargo build --release
../maelstrom/maelstrom test -w broadcast --bin ./target/release/broadcast
