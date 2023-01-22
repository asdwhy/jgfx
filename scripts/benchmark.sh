RUSTFLAGS="-C target-cpu=native"
cargo build -p render
valgrind --tool=callgrind ./target/debug/render
