use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

use super::state::SOURCE;

#[allow(non_snake_case)]
pub fn Preview(cx: Scope) -> Element {
    let source = use_read(&cx, SOURCE);

    let rendered = match use_future(&cx, (source,), |(source,)| async move {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&source, options);

        let mut output = String::new();
        html::push_html(&mut output, parser);
        output
    })
    .value()
    {
        Some(val) => val,
        None => "",
    };

    cx.render(rsx! {
      div {
        class: "border border-gray-300 w-full h-full p-2 prose",
        dangerous_inner_html: "{rendered}"
      }
    })
}
