<h1 align="center">Alchemist's Apprentice</h1>

<p align="center">
  <img height="350" src="https://github.com/Project-Sophon/alchemist-apprentice/blob/main/docs/aa_gh.png?raw=true" alt="The Aclhemist's Apprentice Screenshot"/>
</p>


#
![GitHub](https://img.shields.io/github/license/Project-Sophon/alchemist-apprentice?style=for-the-badge)
![Bevy](https://img.shields.io/badge/bevy-0.10.1-green?style=for-the-badge)

## Play

### Download
You can download the latest native clients from the releases page [here.](https://github.com/Project-Sophon/alchemist-apprentice/releases)

### Browser

#### GitHub Pages
Hosted on GitHub pages: [Play Here](https://project-sophon.github.io/alchemist-apprentice-wasm/)

#### itch.io
https://tsarbomb.itch.io/alchemists-apprentice

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
