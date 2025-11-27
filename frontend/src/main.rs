// src/main.rs
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod services;
mod models;

use components::{login::Login, register::Register, dashboard::Dashboard};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dashboard")]
    Dashboard,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
