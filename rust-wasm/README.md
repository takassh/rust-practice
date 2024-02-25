# What is this?
- the repository to learn rust and wasm based on https://rustwasm.github.io/docs/book/introduction.html


## Memo

### How to set up wasm project using rust?
1. `cargo generate --git https://github.com/rustwasm/wasm-pack-template`
1. `wasm-pack build`
1. `npm init wasm-app www`
1. `npm install` within `www`
1. add wasm dependency with a key into `package.json` within www [package.json](./www/package.json)
1. now you can import wasm using the key
1. `npm install` within `www`
1. `npm run start`

Once you have done above setup, when you make changes your wasm you just run `wasm-pack build` to reflect the change 

### How to test wasm?
- `wasm-pack test --firefox --headless` (you need to install firefox in advance)

### How to debug wasm or js?
- you can use `log!` like `println!` style using `web_sys::console::log_1`
- add `debugger;` in .js file to stop debugger

### How to catch Panics?
- add `utils::set_panic_hook();`

### Why you use rust with wasm, instead of JavaScript?
- For Low-Level Control with High-Level Ergonomics. Comparing to JS, rust with wasm can have low-level control and reliable performance.
- For small .wasm size
- Adoption is easy. Don't need to rewrite everything. You can start from the point which is important for performance.
- You can use JS tooling you love like npm and Webpack.
- For the amenities rust has: cargo, zero-cost abstraction and community!

### What is WebAssembly (wasm)?
- it's a smiple machine model and executable format.
- it's disigned to be portable, compact, and execute at or near native speeds.
- it has two formats to represent the same struct
    - `.wat` text format
    - `.wasm` binary format
- You can convert wat to wasm using [wat to wasm demo](https://webassembly.github.io/wabt/demo/wat2wasm/)
- it has access to Liner Memory.
- it has no assumption on which it can run.

<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
