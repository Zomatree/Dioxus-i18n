# Dioxus-i18n

internationalization for dioxus.

## Example

```rust
fn SubComponent(cx: Scope) -> Element {
    cx.render(rsx! {
        Text { "hello" }
    })
}

fn app(cx: Scope) -> Element {
    let fluent = cx.use_hook(|| {
        let en = "hello = Hello World!";

        let mut langs = HashMap::new();

        let mut bundle = FluentBundle::<FluentResource>::new(vec![langid!("en-GB")]);
        bundle.add_resource(FluentResource::try_new(en.to_string()).unwrap()).unwrap();

        langs.insert(langid!("en-GB"), bundle);

        Cell::new(Some(Fluent::new(langs)))
    });

    cx.render(rsx! {
        TranslationsProvider {
            provider: fluent,
            default_locale: langid!("en-GB"),
            SubComponent {}
        }
    })
}

```
