use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        class: "h-screen flex flex-col items-center justify-center",
        h1 {
          class: "text-5xl font-light",
          "Hello Dioxus"
        }
        p {
          class: "mt-10",
          Link {
            class: "bg-black text-white py-3 px-5 text-xl",
            to: "/app",
            "Go app"
          }
        }
      }
    })
}
