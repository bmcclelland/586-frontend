use yew::prelude::*;
use frontend::{Model,Msg};

fn main() {
    yew::initialize();
    App::<Model>::new()
        .mount_to_body()
        .send_message(Msg::Init)
        ;
    yew::run_loop();
}
