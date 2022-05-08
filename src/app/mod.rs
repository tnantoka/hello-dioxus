use dioxus::prelude::*;

mod editor;
mod preview;
mod state;

use editor::Editor;
use preview::Preview;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
          class: "h-screen flex flex-col",
          nav {
            class: "border-b-2 border-gray-100 py-3 px-4 flex",
            h1 {
               Link {
                 class: "font-semibold",
                 to: "/",
                "Hello Dioxus"
               }
            }
            a {
              class: "ml-auto",
              href: "https://github.com/tnantoka/hello-dioxus",
              "GitHub"
            }
          }
          div {
            class: "py-3 px-4 flex flex-1",
            div {
              class: "flex-1 pr-2",
              Editor {}
            }
            div {
              class: "flex-1 pl-2",
              Preview {}
            }
          }
        }
    })
}
