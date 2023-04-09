
<p align="center">
  <img width="100"src="https://github.com/Project-Sophon/alchemist-apprentice/blob/main/docs/splash.png?raw=true" alt="The Aclhemist's Apprentice"/>
</p>

<p align="center">
  <img height="350" src="https://github.com/Project-Sophon/alchemist-apprentice/blob/main/docs/aa_gh.png?raw=true" alt="The Aclhemist's Apprentice Screenshot"/>
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
$ rustup target install wasm32-unknown-unknown
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\alchemist_apprentice.wasm
```

### Server
There is an included html file to help you serve the game.

Any web server can accomplish this, but for ease we can use the `serve` npm package.

```bash
npx serve .
```