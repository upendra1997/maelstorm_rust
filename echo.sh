sdk use java 11.0.14.9.1-amzn
cargo build --release
../maelstrom/maelstrom test -w echo --bin ./target/release/echo