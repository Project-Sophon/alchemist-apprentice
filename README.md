
<p align="center">
  <img src="https://github.com/Project-Sophon/alchemist-apprentice/blob/main/docs/alchemist_apprentice.png?raw=true" alt="The Aclhemist's Apprentice"/>
</p>

#
![GitHub](https://img.shields.io/github/license/Project-Sophon/alchemist-apprentice?style=for-the-badge)
![Bevy](https://img.shields.io/badge/bevy-0.10.1-green?style=for-the-badge)

## Development

### Run
```bash
cargo run
```

### Build
```bash
cargo build
```

## WASM Build
To build and run the WASM/WebGL version of the game:

### Build

```bash
rustup target install wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\alchemist_apprentice.wasm
```

### Server
There is an included html file to help you serve the game.

Any web server can accomplish this, but for ease we can use the `serve` npm package.

```bash
npx serve .
```