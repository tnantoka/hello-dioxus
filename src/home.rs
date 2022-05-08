use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!{
        div { "Home" }
        Link {
          to: "/app",
          "Go app!"
        }
    })
}
