cargo b -r --features with-base-runtime
cp target/release/trappist-collator bin/base-collator
cargo b -r --features with-trappist-runtime
