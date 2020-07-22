rm ~/rfetch
cargo build --release
ln target/release/rfetch ~/rfetch
echo "Installed"