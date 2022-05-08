use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Preview(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        class: "border border-gray-300 w-full h-full",
        "Preview"
      }
    })
}
 