use dioxus::prelude::*;

mod app;
mod home;

use app::App;
use home::Home;

fn main() {
    dioxus::web::launch(Root);
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", Home {} }
            Route { to: "/app", App {} }
        }
    })
}
