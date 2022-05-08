use dioxus::prelude::*;

pub static SOURCE: Atom<String> = |_| {
    "# hello world

- list item 1
- list item 2

```
code
```

> quote
"
    .to_string()
};
