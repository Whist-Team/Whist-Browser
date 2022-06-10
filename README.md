[![CI](https://github.com/Whist-Team/Whist-Browser/actions/workflows/main.yml/badge.svg)](https://github.com/Whist-Team/Whist-Browser/actions/workflows/main.yml)[![codecov](https://codecov.io/gh/Whist-Team/Whist-Browser/branch/master/graph/badge.svg?token=vn7Nxc9qjb)](https://codecov.io/gh/Whist-Team/Whist-Browser)
# Whist-Browser
Front end client

## Deploy

- Install wasm target: `rustup target add wasm32-unknown-unknown`
- Install trunk: `cargo install --locked trunk`
- Run `trunk serve`

## Development

### Requirements
#### Linux
The following packages are required to build locally.
```shell
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev libssl-dev
```
