use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

struct NavItem {
    link: Route,
    label: String,
    is_active: bool,
    id: u32,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_items = use_state(|| {
        vec![NavItem {
            link: Route::Home,
            label: "Home".to_owned(),
            is_active: false,
            id: 0,
        }]
    });

    html! {}
}
