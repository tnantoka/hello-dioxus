use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx!{
        div { "App" }
        Link {
          to: "/",
          "Go home!"
        }
    })
}
