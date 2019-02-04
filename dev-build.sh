RUSTFLAGS="$RUSTFLAGS -A dead_code"
RUST_BACKTRACE=1
export RUSTFLAGS
export RUST_BACKTRACE
echo "Running with $RUSTFLAGS"
if [ "$1" = "REBUILD_NODES" ]; then
    cd assets/dev
    python3 ./get_nodes.py
    cd ../..
fi
rustfmt ./src/*.rs
if [ "$1" = "RUN" ] || [ "$2" = "RUN" ]; then
    cargo run
else
    cargo build
fi
