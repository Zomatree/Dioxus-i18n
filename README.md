# Dioxus-i18n

internationalization for dioxus.

## Example

```rust
use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::langid;

use dioxus_i18n::{Text, Fluent, use_translations_provider};

fn app(cx: Scope) -> Element {
    use_translations_provider(cx, langid!("en-GB"), || {
        let en = "hello = Hello World!";

        let mut langs = HashMap::new();

        let mut bundle = FluentBundle::<FluentResource>::new(vec![langid!("en-GB")]);
        bundle.add_resource(FluentResource::try_new(en.to_string()).unwrap()).unwrap();

        langs.insert(langid!("en-GB"), bundle);

        Fluent::new(langs)
    });

    let translations = use_translations(cx);

    cx.render(rsx! {
        Text { "hello" }
    })
}
```
