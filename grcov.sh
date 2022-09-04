export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="whist_browser-%p-%m.profraw"
cargo test src
llvm-profdata merge -sparse whist_browser-*.profraw -o whist_browser.profdata
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --ignore "/*" -o ./target/debug/coverage/
xdg-open ./target/debug/coverage/index.html