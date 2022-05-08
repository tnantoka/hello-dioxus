use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
          class: "h-screen flex flex-col",
          nav {
            class: "border-b-2 border-gray-100 py-3 px-4 font-semibold",
            h1 {
               Link {
                 to: "/",
                "Hello Dioxus"
               }
            }
          }
          div {
            class: "py-3 px-4 flex flex-1",
            div {
              class: "flex-1 pr-2",
              textarea {
                class: "border border-gray-300 bg-gray-100 w-full h-full",
              }
            }
            div {
              class: "flex-1 pl-2",
              div {
                class: "border border-gray-300 w-full h-full",
                "Preview"
              }
            }
          }
        }
    })
}
