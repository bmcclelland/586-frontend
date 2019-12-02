#[macro_use] mod macros;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derive_newtype;

mod data; pub use data::*;    
pub mod schema;
