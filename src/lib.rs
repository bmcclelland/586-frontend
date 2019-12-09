#![recursion_limit="256"]

mod domain;
mod model;
mod msg;
mod component;
mod render;
mod authservice;
mod locservice;
mod views;

pub use model::Model;
pub use msg::Msg;
