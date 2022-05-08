use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Editor(cx: Scope) -> Element {
    let source = use_state(&cx, || "# hello world".to_string());

    cx.render(rsx! {
      textarea {
        class: "border border-gray-300 bg-gray-100 w-full h-full",
        oninput: move |evt| source.set(evt.value.clone()),
        value: "{source}",
      }
    })
}
 