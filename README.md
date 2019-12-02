# 586-frontend
Single-page web app for issue tracking. Create projects, workers, and tasks and assign things to things. The project consists of four crates in one workspace.

This is a webassembly application made with the [Yew wasm framework](https://docs.rs/yew/0.4.0/yew/) for Rust. It uses a simple message-based architecture. Output is created with an html! macro that lets you write something similar to regular HTML with static checking and embedded Rust code.

The frontend is thin and leaves all the business logic to the backend.

### Types
#### Model
The client's local context. Implements the Yew traits (Component, Renderable).
#### View
Variant type of different client modes for viewing Model data.
#### Msg
Variant type for Yew's message system. Component::update fields these. Msgs are sent by page events or by other Msgs.
