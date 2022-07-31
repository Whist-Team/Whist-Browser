[![CI](https://github.com/Whist-Team/Whist-Browser/actions/workflows/main.yml/badge.svg)](https://github.com/Whist-Team/Whist-Browser/actions/workflows/main.yml)[![codecov](https://codecov.io/gh/Whist-Team/Whist-Browser/branch/main/graph/badge.svg?token=vn7Nxc9qjb)](https://codecov.io/gh/Whist-Team/Whist-Browser)
# Whist-Browser
Front end client

## Deploy

- Install wasm target: `rustup target add wasm32-unknown-unknown`
- Install trunk or wasm-server-runner : `cargo install --locked trunk` `cargo install --locked wasm-server-runner`
- Run `trunk serve` or `cargo run --target wasm32-unknown-unknown`

## Development

### Requirements
#### Linux
The following packages are required to build locally.
```shell
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev libssl-dev
```
