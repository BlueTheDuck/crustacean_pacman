RUSTFLAGS="$RUSTFLAGS -A dead_code"
RUST_BACKTRACE=1
export RUSTFLAGS
export RUST_BACKTRACE
echo "Running with $RUSTFLAGS"
if [ "$1" = "RUN" ]; then
    cargo run
else
    cargo build
fi
