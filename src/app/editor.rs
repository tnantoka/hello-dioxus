use dioxus::fermi::use_atom_state;
use dioxus::prelude::*;

use super::state::SOURCE;

#[allow(non_snake_case)]
pub fn Editor(cx: Scope) -> Element {
    let source = use_atom_state(&cx, SOURCE);

    cx.render(rsx! {
      textarea {
        class: "border border-gray-300 bg-gray-100 w-full h-full p-2",
        oninput: move |evt| source.set(evt.value.clone()),
        value: "{source}",
      }
    })
}
