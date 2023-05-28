use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{home::Home, image::Image};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/image")]
    Image,
}

#[function_component(App)]
pub fn app() -> Html {
    log::info!("Loading app");
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
        </main>
    }
}
