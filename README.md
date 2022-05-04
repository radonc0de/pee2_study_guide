Couldn't resist trying out [Yew](https://yew.rs/) but also needed to study for finals so I decided to kill two birds with one stone...

This is a study guide for my Principles of Electrical Engineering 2 course at Rutgers written as a Yew Rust webapp that compiles down to [WASM](https://webassembly.org/).

To Run from [Yew Docs](https://yew.rs/docs/tutorial):

Install [trunk](https://crates.io/crates/trunk) if you haven't already with `cargo install trunk`

Next, add WASM as a build target for Rust `rustup target add wasm32-unknown-unknown`

Build the package with `cargo run`

Finally, serve the app with `trunk serve --open`. Open your browser to [http://localhost:8080/](http://localhost:8080/) and view this definitely-not-completely-copied-from-the-tutorial webapp.