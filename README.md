# Telescope 🔭

Telescope is an experimental (but super cool) **Full-Stack WebAssembly Application**

That means both the web frontend AND the backend are running Wasm! It uses [Yew](https://github.com/yewstack/yew) for the frontend, and [Suborbital Atmo](https://github.com/suborbital/atmo) for the backend.

The application is a demo allowing you to view stats about an org's GitHub repos.

## Building
You'll first need to install:
- Cargo/Rust toolchain
	- wasm32-wasi target (`rustup target add wasm32-wasi`)
	- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)
- wasm-bindgen (`cargo install wasm-bindgen-cli`)
- Trunk bundler (`cargo install trunk`)
- [subo](https://github.com/suborbital/subo) (the Suborbital CLI)

Next, create a `static` directory in this repo and add 3 things inside:
- An empty directory called `app`
- A file named `organization` that contains the name of a GitHub org
- Optionally, a file named `webhook` that contains a Discord webhook URL (if you want to try the send report endpoint)

You can then run `make app` to build the frontend and backend

Finally, run `subo dev` to start an Atmo development server and open http://localhost:8080/app to see it in action!

You can use an HTTP client to interact with the other endpoints.