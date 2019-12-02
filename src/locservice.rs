#![allow(unused_imports)]

use crate::domain::*;
use yew::prelude::*;
use stdweb::{
    js, 
    web::Location,
    web::window,
    web::Window,
    web::IEventTarget,
    web::event::PopStateEvent,
};
use serde::{ Serialize, Deserialize };

pub struct LocService;

impl LocService {
    pub fn new() -> Self {
        LocService
    }
    
    pub fn init(&mut self, callback: Callback<()>) {
        window().add_event_listener(
            move |_: PopStateEvent| callback.emit(())
            );
    }

    pub fn get_hash_path(&self) -> Vec<String> {
        window()
            .location().expect("could not get location")
            .hash().unwrap()
            .trim_start_matches("#/")
            .split_terminator('/')
            .map(|s:&str| s.into())
            .collect()
    }

    pub fn set_hash_path(&self, path: String) {
        let window = window();
        js! {
            @{window}.location.hash = "#/" + @{path};
        };
    }
}
