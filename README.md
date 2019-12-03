# 586-frontend
Single-page web app for issue tracking written in Rust. Create projects, workers, and tasks and assign things to things.

This is a webassembly application made with the [Yew wasm framework](https://docs.rs/yew/0.4.0/yew/) for Rust. It uses a simple message-based architecture. Output is created with DSL macros for HTML and Javascript.

Authentication is done with [Auth0](https://auth0.com) using Universal Login.

## Building
Developed using Rust 1.38.0 (stable).

Requires [cargo-web](https://github.com/koute/cargo-web) to build. Execute `cargo-web deploy` and then the servable files will be in target/deploy. Serve from `localhost:8000` to comply with Auth0's whitelists.

## Types
### Model
The client's local context. Implements the Yew traits (Component, Renderable).
### Scene
Variant type of different client modes: view all projects, view single project, etc
### Msg
Variant type for Yew's message system. Component::update fields these. Msgs are sent by page events or by other Msgs.
