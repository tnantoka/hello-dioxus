use dioxus::prelude::*;

mod app;
mod home;

use home::Home;
use app::App;

fn main() {
    dioxus::web::launch(Root);
}

fn Root(cx: Scope) -> Element {
    cx.render(rsx!{
        Router {
            Route { to: "/", Home {} }
            Route { to: "/app", App {} }
        }
    })
}
